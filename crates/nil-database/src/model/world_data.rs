// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::Database;
use crate::error::{Error, Result};
use crate::sql_types::hashed_password::HashedPassword;
use crate::sql_types::id::UserDataId;
use crate::sql_types::world_data_id::WorldDataId;
use crate::sql_types::zoned::Zoned;
use diesel::prelude::*;
use nil_core::world::{World, WorldId};
use nil_util::password::Password;

#[derive(Identifiable, Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::world_data)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(UserData, foreign_key = created_by))]
pub struct WorldData {
  pub id: WorldDataId,
  pub password: Option<HashedPassword>,
  pub description: Option<String>,
  pub created_by: UserDataId,
  pub created_at: Zoned,
  pub updated_at: Zoned,
  pub data: Vec<u8>,
}

impl WorldData {
  #[inline]
  pub fn into_world(self) -> Result<World> {
    Ok(World::load(&self.data)?)
  }
}

/// This can be used to avoid unnecessarily loading the binary data.
/// We should come up with a better name for it.
#[derive(Identifiable, Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::world_data)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(UserData, foreign_key = created_by))]
pub struct WorldDataless {
  pub id: WorldDataId,
  pub password: Option<HashedPassword>,
  pub description: Option<String>,
  pub created_by: UserDataId,
  pub created_at: Zoned,
  pub updated_at: Zoned,
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::world_data)]
pub struct NewWorldData {
  id: WorldDataId,
  password: Option<HashedPassword>,
  description: Option<String>,
  created_by: UserDataId,
  created_at: Zoned,
  updated_at: Zoned,
  data: Vec<u8>,
}

#[bon::bon]
impl NewWorldData {
  #[builder]
  pub fn new(
    #[builder(start_fn)] id: WorldId,
    #[builder(start_fn)] data: Vec<u8>,
    password: Option<&Password>,
    mut description: Option<String>,
    created_by: UserDataId,
  ) -> Result<Self> {
    if let Some(password) = password {
      let pass_len = password.trim().chars().count();
      if !(3..=50).contains(&pass_len) {
        return Err(Error::InvalidPassword);
      }
    }

    if let Some(description) = description.as_mut() {
      while description.len() > 1000 {
        description.pop();
      }
    }

    Ok(Self {
      id: WorldDataId::from(id),
      password: password
        .map(HashedPassword::new)
        .transpose()?,
      description,
      created_by,
      created_at: Zoned::now(),
      updated_at: Zoned::now(),
      data,
    })
  }

  #[inline]
  pub fn data(&self) -> &[u8] {
    &self.data
  }

  #[inline]
  pub fn create(self, database: &Database) -> Result<usize> {
    database.create_world_data(&self)
  }
}
