// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Database;
use crate::error::{Error, Result};
use crate::model::user::{NewUser, User};
use crate::sql_types::id::UserId;
use crate::sql_types::player_id::SqlPlayerId;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use either::Either;

impl Database {
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

  pub fn get_user(&self, player_id: impl Into<SqlPlayerId>) -> Result<User> {
    use crate::schema::user;

    let player_id: SqlPlayerId = player_id.into();
    let result = user::table
      .filter(user::player_id.eq(&player_id))
      .select(User::as_select())
      .first(&mut *self.conn());

    if let Err(DieselError::NotFound) = &result {
      Err(Error::UserNotFound(Either::Left(player_id)))
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

  pub fn user_exists(&self, player_id: impl Into<SqlPlayerId>) -> Result<bool> {
    use crate::schema::user;

    let player_id: SqlPlayerId = player_id.into();
    user::table
      .filter(user::player_id.eq(&player_id))
      .select(user::id)
      .first::<UserId>(&mut *self.conn())
      .optional()
      .map(|it| it.is_some())
      .map_err(Into::into)
  }
}
