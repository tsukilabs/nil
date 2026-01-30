// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Database;
use crate::error::{Error, Result};
use crate::model::user_data::{NewUserData, UserData};
use crate::sql_types::id::UserDataId;
use crate::sql_types::user::User;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use either::Either;

impl Database {
  pub fn create_user_data(&self, new: &NewUserData) -> Result<usize> {
    use crate::schema::user_data::dsl::*;
    let result = diesel::insert_into(user_data)
      .values(new)
      .execute(&mut *self.conn());

    if let Err(DieselError::DatabaseError(kind, _)) = &result
      && let DatabaseErrorKind::UniqueViolation = kind
    {
      Err(Error::UserAlreadyExists(new.user()))
    } else {
      Ok(result?)
    }
  }

  pub fn get_user_data(&self, user: &User) -> Result<UserData> {
    use crate::schema::user_data;
    let result = user_data::table
      .filter(user_data::user.eq(user))
      .select(UserData::as_select())
      .first(&mut *self.conn());

    if let Err(DieselError::NotFound) = &result {
      Err(Error::UserNotFound(Either::Left(user.clone())))
    } else {
      Ok(result?)
    }
  }

  pub fn get_user_data_by_id(&self, id: UserDataId) -> Result<UserData> {
    use crate::schema::user_data;
    let result = user_data::table
      .find(id)
      .select(UserData::as_select())
      .first(&mut *self.conn());

    if let Err(DieselError::NotFound) = &result {
      Err(Error::UserNotFound(Either::Right(id)))
    } else {
      Ok(result?)
    }
  }

  pub fn user_data_exists(&self, user: &User) -> Result<bool> {
    use crate::schema::user_data;
    user_data::table
      .filter(user_data::user.eq(user))
      .select(user_data::id)
      .first::<UserDataId>(&mut *self.conn())
      .optional()
      .map(|it| it.is_some())
      .map_err(Into::into)
  }
}
