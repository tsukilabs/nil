// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Database;
use crate::error::{Error, Result};
use crate::model::game::{Game, GameWithBlob, NewGame};
use crate::sql_types::game_id::GameId;
use crate::sql_types::hashed_password::HashedPassword;
use crate::sql_types::zoned::SqlZoned;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use nil_util::password::Password;
use nil_util::result::WrapOk;

macro_rules! decl_get {
  ($fn_name:ident, $model:ident) => {
    pub fn $fn_name(&self, game_id: impl Into<GameId>) -> Result<$model> {
      use $crate::schema::game;

      let game_id: GameId = game_id.into();
      let result = game::table
        .find(&game_id)
        .select($model::as_select())
        .first(&mut *self.conn());

      if let Err(DieselError::NotFound) = &result {
        Err(Error::GameNotFound(game_id))
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

impl Database {
  decl_get!(get_game, Game);
  decl_get!(get_game_with_blob, GameWithBlob);

  decl_get_all!(get_games, Game);
  decl_get_all!(get_games_with_blob, GameWithBlob);

  pub fn create_game(&self, new: &NewGame) -> Result<usize> {
    use crate::schema::game::dsl::*;
    diesel::insert_into(game)
      .values(new)
      .on_conflict(id)
      .do_update()
      .set((world_blob.eq(new.blob()), updated_at.eq(SqlZoned::now())))
      .execute(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn get_game_password(&self, game_id: impl Into<GameId>) -> Result<Option<HashedPassword>> {
    use crate::schema::game::dsl::*;
    let game_id: GameId = game_id.into();
    game
      .find(&game_id)
      .select(password)
      .first(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn update_game_blob(&self, game_id: impl Into<GameId>, blob: &[u8]) -> Result<usize> {
    use crate::schema::game;
    let game_id: GameId = game_id.into();
    diesel::update(game::table.find(&game_id))
      .set((
        game::world_blob.eq(blob),
        game::updated_at.eq(SqlZoned::now()),
      ))
      .execute(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn verify_game_password(
    &self,
    game_id: impl Into<GameId>,
    password: Option<&Password>,
  ) -> Result<bool> {
    if let Some(hash) = self.get_game_password(game_id)? {
      password
        .filter(|it| !it.trim().is_empty())
        .is_some_and(|it| hash.verify(it))
        .wrap_ok()
    } else {
      Ok(true)
    }
  }
}
