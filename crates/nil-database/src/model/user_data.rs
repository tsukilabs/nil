// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::DatabaseHandle;
use crate::error::{Error, Result};
use crate::sql_types::hashed_password::HashedPassword;
use crate::sql_types::id::UserDataId;
use crate::sql_types::user::User;
use crate::sql_types::zoned::Zoned;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use either::Either;
use nil_util::password::Password;

#[derive(Identifiable, Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::user_data)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserData {
  pub id: UserDataId,
  pub user: User,
  pub password: HashedPassword,
  pub created_at: Zoned,
  pub updated_at: Zoned,
}

impl UserData {
  pub fn get(database: &DatabaseHandle, user: &User) -> Result<Self> {
    use crate::schema::user_data;
    let result = user_data::table
      .filter(user_data::user.eq(user))
      .select(Self::as_select())
      .first(&mut *database.conn());

    if let Err(DieselError::NotFound) = &result {
      Err(Error::UserNotFound(Either::Left(user.clone())))
    } else {
      Ok(result?)
    }
  }

  pub fn get_by_id(database: &DatabaseHandle, id: UserDataId) -> Result<Self> {
    use crate::schema::user_data;
    let result = user_data::table
      .find(id)
      .select(Self::as_select())
      .first(&mut *database.conn());

    if let Err(DieselError::NotFound) = &result {
      Err(Error::UserNotFound(Either::Right(id)))
    } else {
      Ok(result?)
    }
  }

  pub fn exists(database: &DatabaseHandle, user: &User) -> Result<bool> {
    use crate::schema::user_data;
    user_data::table
      .filter(user_data::user.eq(user))
      .select(user_data::id)
      .first::<UserDataId>(&mut *database.conn())
      .optional()
      .map(|it| it.is_some())
      .map_err(Into::into)
  }

  #[inline]
  pub fn verify_password(&self, password: &Password) -> bool {
    self.password.verify(password)
  }
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::user_data)]
pub struct NewUserData {
  user: User,
  password: HashedPassword,
  created_at: Zoned,
  updated_at: Zoned,
}

impl NewUserData {
  pub fn new(user: User, password: &Password) -> Result<Self> {
    let user_len = user.trim().chars().count();
    let pass_len = password.trim().chars().count();
    if !(1..=20).contains(&user_len) {
      return Err(Error::InvalidUsername(user));
    } else if !(3..=50).contains(&pass_len) {
      return Err(Error::InvalidPassword);
    }

    Ok(Self {
      user,
      password: HashedPassword::new(password)?,
      created_at: Zoned::now(),
      updated_at: Zoned::now(),
    })
  }

  pub fn create(self, database: &DatabaseHandle) -> Result<usize> {
    use crate::schema::user_data::dsl::*;
    let result = diesel::insert_into(user_data)
      .values(&self)
      .execute(&mut *database.conn());

    if let Err(DieselError::DatabaseError(kind, _)) = &result
      && let DatabaseErrorKind::UniqueViolation = kind
    {
      Err(Error::UserAlreadyExists(self.user))
    } else {
      Ok(result?)
    }
  }
}
