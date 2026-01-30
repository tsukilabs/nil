// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::Database;
use crate::error::{Error, Result};
use crate::sql_types::hashed_password::HashedPassword;
use crate::sql_types::id::UserDataId;
use crate::sql_types::user::User;
use crate::sql_types::zoned::Zoned;
use diesel::prelude::*;
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

  pub fn user(&self) -> User {
    self.user.clone()
  }

  pub fn create(self, database: &Database) -> Result<usize> {
    database.create_user_data(&self)
  }
}
