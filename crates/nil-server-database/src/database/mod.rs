// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod game;
mod user;

use crate::error::Result;
use crate::migration::run_pending_migrations;
use crate::model::game::{Game, GameWithBlob, NewGame};
use crate::model::user::{NewUser, User};
use crate::sql_types::duration::Duration;
use crate::sql_types::game_id::GameId;
use crate::sql_types::hashed_password::HashedPassword;
use crate::sql_types::id::UserId;
use crate::sql_types::player_id::PlayerId;
use diesel::prelude::*;
use itertools::Itertools;
use nil_crypto::password::Password;
use std::sync::Arc;
use std::sync::nonpoison::{Mutex, MutexGuard};
use tokio::task::spawn_blocking;

#[must_use]
#[derive(Clone)]
pub struct BlockingDatabase(Arc<Mutex<SqliteConnection>>);

impl BlockingDatabase {
  fn new(url: &str) -> Result<Self> {
    let mut conn = SqliteConnection::establish(url)?;
    run_pending_migrations(&mut conn);
    Ok(Self(Arc::new(Mutex::new(conn))))
  }

  fn conn(&self) -> MutexGuard<'_, SqliteConnection> {
    self.0.lock()
  }
}

#[must_use]
#[derive(Clone)]
pub struct Database(BlockingDatabase);

impl Database {
  pub async fn new(url: &str) -> Result<Self> {
    let url = url.to_owned();
    spawn_blocking(move || BlockingDatabase::new(&url).map(Self)).await?
  }

  pub fn blocking(&self) -> BlockingDatabase {
    self.0.clone()
  }

  async fn with_blocking<F, T>(&self, f: F) -> Result<T>
  where
    F: FnOnce(BlockingDatabase) -> Result<T> + Send + 'static,
    T: Send + 'static,
  {
    let blocking = self.blocking();
    spawn_blocking(move || f(blocking)).await?
  }

  pub async fn count_games(&self) -> Result<i64> {
    self
      .with_blocking(|db| db.count_games())
      .await
  }

  pub async fn count_games_by_user(&self, id: impl Into<PlayerId>) -> Result<i64> {
    let id: PlayerId = id.into();
    self
      .with_blocking(move |db| db.count_games_by_user(id))
      .await
  }

  pub async fn count_games_by_user_id(&self, id: impl Into<UserId>) -> Result<i64> {
    let id: UserId = id.into();
    self
      .with_blocking(move |db| db.count_games_by_user_id(id))
      .await
  }

  pub async fn create_game(&self, new: NewGame) -> Result<usize> {
    self
      .with_blocking(move |db| db.create_game(&new))
      .await
  }

  pub async fn create_user(&self, new: NewUser) -> Result<usize> {
    self
      .with_blocking(move |db| db.create_user(&new))
      .await
  }

  pub async fn delete_game(&self, id: impl Into<GameId>) -> Result<usize> {
    let id: GameId = id.into();
    self
      .with_blocking(move |db| db.delete_game(id))
      .await
  }

  pub async fn delete_games<I>(&self, games: I) -> Result<usize>
  where
    I: IntoIterator<Item = GameId>,
  {
    let games = games.into_iter().collect_vec();
    self
      .with_blocking(move |db| db.delete_games(&games))
      .await
  }

  pub async fn game_exists(&self, id: impl Into<GameId>) -> Result<bool> {
    let id: GameId = id.into();
    self
      .with_blocking(move |db| db.game_exists(id))
      .await
  }

  pub async fn get_game(&self, id: impl Into<GameId>) -> Result<Game> {
    let id: GameId = id.into();
    self
      .with_blocking(move |db| db.get_game(id))
      .await
  }

  pub async fn get_game_creator(&self, id: impl Into<GameId>) -> Result<PlayerId> {
    let id: GameId = id.into();
    self
      .with_blocking(move |db| db.get_game_creator(id))
      .await
  }

  pub async fn get_game_ids(&self) -> Result<Vec<GameId>> {
    self
      .with_blocking(|db| db.get_game_ids())
      .await
  }

  pub async fn get_game_password(&self, id: impl Into<GameId>) -> Result<Option<HashedPassword>> {
    let id: GameId = id.into();
    self
      .with_blocking(move |db| db.get_game_password(id))
      .await
  }

  pub async fn get_game_round_duration(&self, id: impl Into<GameId>) -> Result<Option<Duration>> {
    let id: GameId = id.into();
    self
      .with_blocking(move |db| db.get_game_round_duration(id))
      .await
  }

  pub async fn get_game_with_blob(&self, id: impl Into<GameId>) -> Result<GameWithBlob> {
    let id: GameId = id.into();
    self
      .with_blocking(move |db| db.get_game_with_blob(id))
      .await
  }

  pub async fn get_games(&self) -> Result<Vec<Game>> {
    self.with_blocking(|db| db.get_games()).await
  }

  pub async fn get_games_with_blob(&self) -> Result<Vec<GameWithBlob>> {
    self
      .with_blocking(|db| db.get_games_with_blob())
      .await
  }

  pub async fn get_user(&self, id: impl Into<PlayerId>) -> Result<User> {
    let id: PlayerId = id.into();
    self
      .with_blocking(move |db| db.get_user(id))
      .await
  }

  pub async fn get_user_by_id(&self, id: impl Into<UserId>) -> Result<User> {
    let id: UserId = id.into();
    self
      .with_blocking(move |db| db.get_user_by_id(id))
      .await
  }

  pub async fn get_user_id(&self, id: impl Into<PlayerId>) -> Result<UserId> {
    let id: PlayerId = id.into();
    self
      .with_blocking(move |db| db.get_user_id(id))
      .await
  }

  pub async fn get_user_player_id(&self, id: impl Into<UserId>) -> Result<PlayerId> {
    let id: UserId = id.into();
    self
      .with_blocking(move |db| db.get_user_player_id(id))
      .await
  }

  pub async fn update_game_blob(&self, id: impl Into<GameId>, blob: Vec<u8>) -> Result<usize> {
    let id: GameId = id.into();
    self
      .with_blocking(move |db| db.update_game_blob(id, &blob))
      .await
  }

  pub async fn user_exists(&self, id: impl Into<PlayerId>) -> Result<bool> {
    let id: PlayerId = id.into();
    self
      .with_blocking(move |db| db.user_exists(&id))
      .await
  }

  pub async fn verify_game_password(
    &self,
    id: impl Into<GameId>,
    password: Option<Password>,
  ) -> Result<bool> {
    let id: GameId = id.into();
    self
      .with_blocking(move |db| db.verify_game_password(id, password.as_ref()))
      .await
  }

  pub async fn was_game_created_by(
    &self,
    game_id: impl Into<GameId>,
    player_id: impl Into<PlayerId>,
  ) -> Result<bool> {
    let game_id: GameId = game_id.into();
    let player_id: PlayerId = player_id.into();
    self
      .with_blocking(move |db| db.was_game_created_by(game_id, &player_id))
      .await
  }
}
