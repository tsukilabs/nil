// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::response::{MaybeResponse, from_err};
use crate::server::remote;
use crate::{VERSION, res};
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
use nil_core::world::{World, WorldId, WorldOptions};
use nil_crypto::password::Password;
use nil_database::Database;
use nil_database::model::game::NewGame;
use nil_database::sql_types::player_id::SqlPlayerId;
use nil_database::sql_types::version::SqlVersion;
use nil_server_types::ServerKind;
use semver::{Prerelease, Version};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::spawn_blocking;

#[derive(Clone)]
pub(crate) struct App {
  server_kind: ServerKind,
  worlds: Arc<DashMap<WorldId, Arc<RwLock<World>>>>,
  database: Option<Database>,
}

#[bon::bon]
impl App {
  pub fn new_local(world: World) -> Self {
    let id = world.config().id();
    let app = Self {
      server_kind: ServerKind::Local { id },
      worlds: Arc::new(DashMap::new()),
      database: None,
    };

    app
      .worlds
      .insert(id, Arc::new(RwLock::new(world)));

    app
  }

  pub fn new_remote(database_url: &str) -> Result<Self> {
    let worlds = Arc::new(DashMap::new());
    let database = Database::new(database_url)?;

    let mut invalid_games = Vec::new();

    let now = Zoned::now();
    let version = Version::parse(VERSION)?;
    let minor = if version.major == 0 { version.minor } else { 0 };
    let version_cmp = semver::Comparator {
      op: semver::Op::Caret,
      major: version.major,
      minor: Some(minor),
      patch: Some(0),
      pre: Prerelease::EMPTY,
    };

    for game in database.get_games_with_blob()? {
      let id = game.id;
      if version_cmp.matches(&game.server_version)
        && let Ok(span) = game.updated_at.until(&now)
        && span.get_days() <= 90
        && let Ok(mut world) = game.into_world()
      {
        let world_id = world.config().id();
        let database = database.clone();
        world.on_next_round(remote::on_next_round(database));

        worlds.insert(world_id, Arc::new(RwLock::new(world)));
      } else {
        invalid_games.push(id);
      }
    }

    database.delete_games(&invalid_games)?;

    Ok(Self {
      server_kind: ServerKind::Remote,
      worlds,
      database: Some(database),
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

  /// # Panics
  ///
  /// Panics if the server is not remote.
  #[builder]
  pub(crate) fn create_remote(
    &self,
    #[builder(start_fn)] options: &WorldOptions,
    #[builder(into)] player_id: SqlPlayerId,
    #[builder(into)] world_description: Option<String>,
    world_password: Option<&Password>,
    #[builder(into)] server_version: SqlVersion,
  ) -> Result<WorldId> {
    let db = self.database();
    let user = db.get_user(player_id)?;

    let mut world = World::try_from(options)?;
    let world_id = world.config().id();
    let blob = world.to_bytes()?;

    NewGame::builder(world_id, blob)
      .created_by(user.id)
      .maybe_description(world_description)
      .maybe_password(world_password)
      .server_version(server_version)
      .build()?
      .create(&db)?;

    let db = db.clone();
    world.on_next_round(remote::on_next_round(db));

    self
      .worlds
      .insert(world_id, Arc::new(RwLock::new(world)));

    Ok(world_id)
  }

  pub(crate) fn get(&self, id: WorldId) -> Result<Arc<RwLock<World>>> {
    self
      .worlds
      .get(&id)
      .map(|world| Arc::clone(&world))
      .ok_or_else(|| Error::InvalidWorld(id))
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
          Err(_) => Either::Right(res!(INTERNAL_SERVER_ERROR)),
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
          Err(_) => Either::Right(res!(INTERNAL_SERVER_ERROR)),
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
