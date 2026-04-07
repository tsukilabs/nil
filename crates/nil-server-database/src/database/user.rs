// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::BlockingDatabase;
use crate::error::{Error, Result};
use crate::model::user::{NewUser, User};
use crate::sql_types::id::UserId;
use crate::sql_types::player_id::PlayerId;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use either::Either;

impl BlockingDatabase {
  pub fn count_games_by_user(&self, id: PlayerId) -> Result<i64> {
    self.count_games_by_user_id(self.get_user_id(id)?)
  }

  pub fn count_games_by_user_id(&self, id: UserId) -> Result<i64> {
    use crate::schema::game;

    game::table
      .filter(game::created_by.eq(id))
      .count()
      .get_result(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn create_user(&self, new: &NewUser) -> Result<usize> {
    use crate::schema::user::dsl::*;

    let result = diesel::insert_into(user)
      .values(new)
      .execute(&mut *self.conn());

    if let Err(DieselError::DatabaseError(kind, _)) = &result
      && let DatabaseErrorKind::UniqueViolation = kind
    {
      Err(Error::UserAlreadyExists(new.player_id()))
    } else {
      Ok(result?)
    }
  }

  pub fn get_user(&self, id: PlayerId) -> Result<User> {
    use crate::schema::user;

    let result = user::table
      .filter(user::player_id.eq(&id))
      .select(User::as_select())
      .first(&mut *self.conn());

    if let Err(DieselError::NotFound) = &result {
      Err(Error::UserNotFound(Either::Left(id)))
    } else {
      Ok(result?)
    }
  }

  pub fn get_user_by_id(&self, id: UserId) -> Result<User> {
    use crate::schema::user;

    let result = user::table
      .find(id)
      .select(User::as_select())
      .first(&mut *self.conn());

    if let Err(DieselError::NotFound) = &result {
      Err(Error::UserNotFound(Either::Right(id)))
    } else {
      Ok(result?)
    }
  }

  pub fn get_user_id(&self, id: PlayerId) -> Result<UserId> {
    use crate::schema::user;

    let result = user::table
      .filter(user::player_id.eq(&id))
      .select(user::id)
      .first(&mut *self.conn());

    if let Err(DieselError::NotFound) = &result {
      Err(Error::UserNotFound(Either::Left(id)))
    } else {
      Ok(result?)
    }
  }

  pub fn get_user_player_id(&self, id: UserId) -> Result<PlayerId> {
    use crate::schema::user;

    let result = user::table
      .find(id)
      .select(user::player_id)
      .first::<PlayerId>(&mut *self.conn());

    if let Err(DieselError::NotFound) = &result {
      Err(Error::UserNotFound(Either::Right(id)))
    } else {
      Ok(result?)
    }
  }

  pub fn user_exists(&self, id: &PlayerId) -> Result<bool> {
    use crate::schema::user;
    use diesel::dsl::{exists, select};

    select(exists(user::table.filter(user::player_id.eq(id))))
      .get_result(&mut *self.conn())
      .map_err(Into::into)
  }
}
