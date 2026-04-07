// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::BlockingDatabase;
use crate::error::{Error, Result};
use crate::model::game::{Game, GameWithBlob, NewGame};
use crate::sql_types::duration::Duration;
use crate::sql_types::game_id::GameId;
use crate::sql_types::hashed_password::HashedPassword;
use crate::sql_types::id::UserId;
use crate::sql_types::player_id::PlayerId;
use crate::sql_types::zoned::Zoned;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use nil_crypto::password::Password;
use tap::Pipe;

macro_rules! decl_get {
  ($fn_name:ident, $model:ident) => {
    pub fn $fn_name(&self, id: GameId) -> Result<$model> {
      use $crate::schema::game;

      let result = game::table
        .find(&id)
        .select($model::as_select())
        .first(&mut *self.conn());

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
    pub fn $fn_name(&self) -> Result<Vec<$model>> {
      use crate::schema::game::dsl::*;
      game
        .select($model::as_select())
        .load(&mut *self.conn())
        .map_err(Into::into)
    }
  };
}

impl BlockingDatabase {
  decl_get!(get_game, Game);
  decl_get!(get_game_with_blob, GameWithBlob);

  decl_get_all!(get_games, Game);
  decl_get_all!(get_games_with_blob, GameWithBlob);

  pub fn count_games(&self) -> Result<i64> {
    use crate::schema::game;

    game::table
      .count()
      .get_result(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn create_game(&self, new: &NewGame) -> Result<usize> {
    use crate::schema::game;

    diesel::insert_into(game::table)
      .values(new)
      .on_conflict(game::id)
      .do_update()
      .set((
        game::world_blob.eq(new.blob()),
        game::updated_at.eq(Zoned::now()),
      ))
      .execute(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn delete_game(&self, id: GameId) -> Result<usize> {
    use crate::schema::game;

    diesel::delete(game::table.find(id))
      .execute(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn delete_games(&self, ids: &[GameId]) -> Result<usize> {
    use crate::schema::game;

    if ids.is_empty() {
      Ok(0)
    } else {
      diesel::delete(game::table.filter(game::id.eq_any(ids)))
        .execute(&mut *self.conn())
        .map_err(Into::into)
    }
  }

  pub fn game_exists(&self, id: GameId) -> Result<bool> {
    use crate::schema::game;
    use diesel::dsl::{exists, select};

    select(exists(game::table.find(id)))
      .get_result(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn get_game_creator(&self, id: GameId) -> Result<PlayerId> {
    use crate::schema::game;

    let user_id = game::table
      .find(&id)
      .select(game::created_by)
      .first::<UserId>(&mut *self.conn())?;

    self.get_user_player_id(user_id)
  }

  pub fn get_game_ids(&self) -> Result<Vec<GameId>> {
    use crate::schema::game;

    game::table
      .select(game::id)
      .load(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn get_game_password(&self, id: GameId) -> Result<Option<HashedPassword>> {
    use crate::schema::game;

    game::table
      .find(&id)
      .select(game::password)
      .first(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn get_game_round_duration(&self, id: GameId) -> Result<Option<Duration>> {
    use crate::schema::game;

    game::table
      .find(&id)
      .select(game::round_duration)
      .first(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn update_game_blob(&self, id: GameId, blob: &[u8]) -> Result<usize> {
    use crate::schema::game;

    diesel::update(game::table.find(&id))
      .set((game::world_blob.eq(blob), game::updated_at.eq(Zoned::now())))
      .execute(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn verify_game_password(&self, id: GameId, password: Option<&Password>) -> Result<bool> {
    if let Some(hash) = self.get_game_password(id)? {
      password
        .filter(|it| !it.trim().is_empty())
        .is_some_and(|it| matches!(hash.verify(it), Ok(true)))
        .pipe(Ok)
    } else {
      Ok(true)
    }
  }

  pub fn was_game_created_by(&self, game_id: GameId, player_id: &PlayerId) -> Result<bool> {
    Ok(&self.get_game_creator(game_id)? == player_id)
  }
}
