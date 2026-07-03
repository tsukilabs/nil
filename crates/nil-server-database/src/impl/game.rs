// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::model::game::{Game, GameWithBlob, NewGame};
use crate::sql_types::duration::db_Duration;
use crate::sql_types::game_id::GameId;
use crate::sql_types::hashed_password::HashedPassword;
use crate::sql_types::id::UserId;
use crate::sql_types::player_id::db_PlayerId;
use crate::sql_types::zoned::db_Zoned;
use crate::{Database, conn};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::RunQueryDsl;
use nil_crypto::password::Password;
use tap::Pipe;

macro_rules! decl_get {
  ($fn_name:ident, $model:ident) => {
    pub async fn $fn_name(&self, id: impl Into<GameId>) -> Result<$model> {
      use $crate::schema::game;

      let id: GameId = id.into();
      let result = game::table
        .find(&id)
        .select($model::as_select())
        .first(conn!(self))
        .await;

      if let Err(DieselError::NotFound) = &result {
        Err(Error::GameNotFound(id))
      } else {
        Ok(result?)
      }
    }
  };
}

macro_rules! decl_get_all {
  ($fn_name:ident, $model:ident) => {
    pub async fn $fn_name(&self) -> Result<Vec<$model>> {
      use $crate::schema::game;

      game::table
        .select($model::as_select())
        .load(conn!(self))
        .await
        .map_err(Into::into)
    }
  };
}

impl Database {
  decl_get!(get_game, Game);
  decl_get!(get_game_with_blob, GameWithBlob);

  decl_get_all!(get_games, Game);
  decl_get_all!(get_games_with_blob, GameWithBlob);

  pub async fn count_games(&self) -> Result<i64> {
    use crate::schema::game;

    game::table
      .count()
      .get_result(conn!(self))
      .await
      .map_err(Into::into)
  }

  pub async fn create_game(&self, new: &NewGame) -> Result<usize> {
    use crate::schema::game;

    diesel::insert_into(game::table)
      .values(new)
      .on_conflict(game::id)
      .do_update()
      .set((
        game::world_blob.eq(new.blob()),
        game::updated_at.eq(db_Zoned::now()),
      ))
      .execute(conn!(self))
      .await
      .map_err(Into::into)
  }

  pub async fn delete_game(&self, id: impl Into<GameId>) -> Result<usize> {
    use crate::schema::game;

    let id: GameId = id.into();
    diesel::delete(game::table.find(id))
      .execute(conn!(self))
      .await
      .map_err(Into::into)
  }

  pub async fn delete_games(&self, ids: &[GameId]) -> Result<usize> {
    use crate::schema::game;

    if ids.is_empty() {
      Ok(0)
    } else {
      diesel::delete(game::table.filter(game::id.eq_any(ids)))
        .execute(conn!(self))
        .await
        .map_err(Into::into)
    }
  }

  pub async fn game_exists(&self, id: GameId) -> Result<bool> {
    use crate::schema::game;
    use diesel::dsl::{exists, select};

    select(exists(game::table.find(id)))
      .get_result(conn!(self))
      .await
      .map_err(Into::into)
  }

  pub async fn get_game_owner(&self, id: GameId) -> Result<db_PlayerId> {
    use crate::schema::game;

    let user_id = game::table
      .find(&id)
      .select(game::created_by)
      .first::<UserId>(conn!(self))
      .await?;

    self.get_user_player_id(user_id).await
  }

  pub async fn get_game_ids(&self) -> Result<Vec<GameId>> {
    use crate::schema::game;

    game::table
      .select(game::id)
      .load(conn!(self))
      .await
      .map_err(Into::into)
  }

  pub async fn get_game_password(&self, id: GameId) -> Result<Option<HashedPassword>> {
    use crate::schema::game;

    let result = game::table
      .find(&id)
      .select(game::password)
      .first(conn!(self))
      .await;

    if let Err(DieselError::NotFound) = &result {
      Err(Error::GameNotFound(id))
    } else {
      Ok(result?)
    }
  }

  pub async fn get_game_round_duration(&self, id: GameId) -> Result<Option<db_Duration>> {
    use crate::schema::game;

    let result = game::table
      .find(&id)
      .select(game::round_duration)
      .first(conn!(self))
      .await;

    if let Err(DieselError::NotFound) = &result {
      Err(Error::GameNotFound(id))
    } else {
      Ok(result?)
    }
  }

  pub async fn update_game_blob(&self, id: GameId, blob: &[u8]) -> Result<usize> {
    use crate::schema::game;

    let n = diesel::update(game::table.find(&id))
      .set((
        game::world_blob.eq(blob),
        game::updated_at.eq(db_Zoned::now()),
      ))
      .execute(conn!(self))
      .await?;

    if n == 0 { Err(Error::GameNotFound(id)) } else { Ok(n) }
  }

  pub async fn verify_game_password(
    &self,
    id: impl Into<GameId>,
    password: Option<&Password>,
  ) -> Result<bool> {
    let id: GameId = id.into();
    if let Some(hash) = self.get_game_password(id).await? {
      password
        .filter(|it| !it.trim().is_empty())
        .is_some_and(|it| matches!(hash.verify(it), Ok(true)))
        .pipe(Ok)
    } else {
      Ok(true)
    }
  }

  pub async fn is_game_owned_by(
    &self,
    game_id: impl Into<GameId>,
    player_id: impl Into<db_PlayerId>,
  ) -> Result<bool> {
    let game_id: GameId = game_id.into();
    let player_id: db_PlayerId = player_id.into();
    let owner = self.get_game_owner(game_id).await?;
    Ok(owner == player_id)
  }
}
