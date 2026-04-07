// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::response::{MaybeResponse, from_err};
use crate::server::{remote, spawn_round_duration_task};
use crate::{VERSION, env, res};
use dashmap::DashMap;
use either::Either;
use jiff::Zoned;
use nil_core::chat::Chat;
use nil_core::continent::Continent;
use nil_core::npc::bot::BotManager;
use nil_core::npc::precursor::PrecursorManager;
use nil_core::player::PlayerManager;
use nil_core::ranking::Ranking;
use nil_core::report::ReportManager;
use nil_core::round::Round;
use nil_core::world::config::WorldId;
use nil_core::world::{World, WorldOptions};
use nil_crypto::password::Password;
use nil_server_database::Database;
use nil_server_database::model::game::{GameWithBlob, NewGame};
use nil_server_database::sql_types::player_id::PlayerId;
use nil_server_types::ServerKind;
use nil_server_types::round::RoundDuration;
use semver::{Prerelease, Version};
use std::num::NonZeroU16;
use std::sync::{Arc, Weak};
use std::time::Duration;
use tap::TryConv;
use tokio::sync::RwLock;
use tokio::task::{spawn, spawn_blocking};

#[derive(Clone)]
pub(crate) struct App {
  server_kind: ServerKind,
  database: Option<Database>,
  worlds: Arc<DashMap<WorldId, Arc<RwLock<World>>>>,
  world_limit: NonZeroU16,
  world_limit_per_user: NonZeroU16,
}

#[bon::bon]
impl App {
  pub fn new_local(world: World) -> Self {
    let id = world.config().id();
    let app = Self {
      server_kind: ServerKind::Local { id },
      database: None,
      worlds: Arc::new(DashMap::new()),
      world_limit: NonZeroU16::MIN,
      world_limit_per_user: NonZeroU16::MIN,
    };

    app
      .worlds
      .insert(id, Arc::new(RwLock::new(world)));

    app
  }

  pub async fn new_remote(database_url: &str) -> Result<Self> {
    let worlds = Arc::new(DashMap::new());
    let database = Database::new(database_url).await?;

    let mut invalid_games = Vec::new();

    for game_id in database.get_game_ids().await? {
      if let Ok(game) = database.get_game_with_blob(game_id).await
        && has_valid_version(&game)
        && has_valid_age(&game)
        && let Ok(world) = game.to_world()
      {
        let world_id = world.config().id();
        let round_id = world.round().id();
        let is_round_idle = world.round().is_idle();

        let database = database.clone();
        let world = Arc::new(RwLock::new(world));
        let weak_world = Arc::downgrade(&world);

        if let Some(round_duration) = game.round_duration
          && !is_round_idle
        {
          spawn(spawn_round_duration_task(
            round_id,
            Weak::clone(&weak_world),
            round_duration.into(),
          ));
        }

        world.write().await.on_next_round(
          remote::on_next_round()
            .database(database)
            .weak_world(weak_world)
            .maybe_round_duration(game.round_duration)
            .call(),
        );

        worlds.insert(world_id, world);
      } else {
        tracing::warn!(invalid_game = %game_id);
        invalid_games.push(game_id);
      }
    }

    database.delete_games(invalid_games).await?;

    Ok(Self {
      server_kind: ServerKind::Remote,
      database: Some(database),
      worlds,
      world_limit: env::remote_world_limit(),
      world_limit_per_user: env::remote_world_limit_per_user(),
    })
  }

  #[inline]
  pub fn server_kind(&self) -> ServerKind {
    self.server_kind
  }

  /// # Panics
  ///
  /// Panics if the server is not remote.
  pub fn database(&self) -> Database {
    if let ServerKind::Remote = self.server_kind
      && let Some(database) = &self.database
    {
      database.clone()
    } else {
      panic!("Not a remote server")
    }
  }

  pub fn world_ids(&self) -> Vec<WorldId> {
    self
      .worlds
      .iter()
      .map(|entry| *entry.key())
      .collect()
  }

  #[inline]
  pub fn world_limit(&self) -> u16 {
    self.world_limit.get()
  }

  #[inline]
  pub fn world_limit_per_user(&self) -> u16 {
    self.world_limit_per_user.get()
  }

  /// Creates a new remote world with the given options.
  ///
  /// # Panics
  ///
  /// Panics if the server is not remote.
  #[builder]
  pub(crate) async fn create_remote(
    &self,
    #[builder(start_fn)] options: &WorldOptions,
    #[builder(into)] player_id: PlayerId,
    #[builder(into)] world_description: Option<String>,
    world_password: Option<Password>,
    round_duration: Option<RoundDuration>,
    server_version: Version,
  ) -> Result<WorldId> {
    self
      .check_remote_world_limit(player_id.clone())
      .await?;

    let database = self.database();
    let user = database.get_user(player_id).await?;

    let world = World::try_from(options)?;
    let world_id = world.config().id();
    let blob = world.to_bytes()?;

    NewGame::builder(world_id, blob)
      .created_by(user.id)
      .maybe_description(world_description)
      .maybe_password(world_password)
      .maybe_round_duration(round_duration)
      .server_version(server_version)
      .build()
      .await?
      .create(&database)
      .await?;

    let database = database.clone();
    let world = Arc::new(RwLock::new(world));

    world.write().await.on_next_round(
      remote::on_next_round()
        .database(database)
        .weak_world(Arc::downgrade(&world))
        .maybe_round_duration(round_duration)
        .call(),
    );

    self.worlds.insert(world_id, world);

    Ok(world_id)
  }

  /// Checks if the player can create a new remote world.
  async fn check_remote_world_limit(&self, player: PlayerId) -> Result<()> {
    let database = self.database();

    let limit = i64::from(self.world_limit.get());
    if database.count_games().await? >= limit {
      return Err(Error::WorldLimitReached);
    }

    let limit_per_user = i64::from(self.world_limit_per_user.get());
    if database.count_games_by_user(player).await? >= limit_per_user {
      return Err(Error::WorldLimitReached);
    }

    Ok(())
  }

  pub(crate) fn get(&self, id: WorldId) -> Result<Arc<RwLock<World>>> {
    self
      .worlds
      .get(&id)
      .map(|world| Arc::clone(&world))
      .ok_or_else(|| Error::WorldNotFound(id))
  }

  pub(crate) fn remove(&self, id: WorldId) -> Option<Arc<RwLock<World>>> {
    self.worlds.remove(&id).map(|it| it.1)
  }

  pub async fn world<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&World) -> T,
  {
    match self.get(id) {
      Ok(world) => Either::Left(f(&*world.read().await)),
      Err(err) => Either::Right(from_err(err)),
    }
  }

  pub async fn world_mut<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&mut World) -> T,
  {
    match self.get(id) {
      Ok(world) => Either::Left(f(&mut *world.write().await)),
      Err(err) => Either::Right(from_err(err)),
    }
  }

  pub async fn world_blocking<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&World) -> T + Send + Sync + 'static,
    T: Send + Sync + 'static,
  {
    match self.get(id) {
      Ok(world) => {
        match spawn_blocking(move || f(&world.blocking_read())).await {
          Ok(value) => Either::Left(value),
          Err(err) => {
            tracing::error!(message = %err, error = ?err);
            Either::Right(res!(INTERNAL_SERVER_ERROR))
          }
        }
      }
      Err(err) => Either::Right(from_err(err)),
    }
  }

  pub async fn world_blocking_mut<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&mut World) -> T + Send + Sync + 'static,
    T: Send + Sync + 'static,
  {
    match self.get(id) {
      Ok(world) => {
        match spawn_blocking(move || f(&mut world.blocking_write())).await {
          Ok(value) => Either::Left(value),
          Err(err) => {
            tracing::error!(message = %err, error = ?err);
            Either::Right(res!(INTERNAL_SERVER_ERROR))
          }
        }
      }
      Err(err) => Either::Right(from_err(err)),
    }
  }

  pub async fn bot_manager<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&BotManager) -> T,
  {
    self
      .world(id, |world| f(world.bot_manager()))
      .await
  }

  pub async fn chat<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&Chat) -> T,
  {
    self.world(id, |world| f(world.chat())).await
  }

  pub async fn continent<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&Continent) -> T,
  {
    self
      .world(id, |world| f(world.continent()))
      .await
  }

  pub async fn player_manager<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&PlayerManager) -> T,
  {
    self
      .world(id, |world| f(world.player_manager()))
      .await
  }

  pub async fn precursor_manager<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&PrecursorManager) -> T,
  {
    self
      .world(id, |world| f(world.precursor_manager()))
      .await
  }

  pub async fn ranking<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&Ranking) -> T,
  {
    self
      .world(id, |world| f(world.ranking()))
      .await
  }

  pub async fn report_manager<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&ReportManager) -> T,
  {
    self
      .world(id, |world| f(world.report()))
      .await
  }

  pub async fn round<F, T>(&self, id: WorldId, f: F) -> MaybeResponse<T>
  where
    F: FnOnce(&Round) -> T,
  {
    self
      .world(id, |world| f(world.round()))
      .await
  }
}

fn has_valid_version(game: &GameWithBlob) -> bool {
  let Ok(version) = Version::parse(VERSION) else {
    unreachable!("Current version should always be valid")
  };

  let minor = if version.major == 0 { version.minor } else { 0 };
  let version_cmp = semver::Comparator {
    op: semver::Op::Caret,
    major: version.major,
    minor: Some(minor),
    patch: Some(0),
    pre: Prerelease::EMPTY,
  };

  version_cmp.matches(&game.server_version)
}

fn has_valid_age(game: &GameWithBlob) -> bool {
  let Ok(duration) = game
    .updated_at
    .duration_until(&Zoned::now())
    .try_conv::<Duration>()
  else {
    return false;
  };

  duration <= Duration::from_days(30)
}
