// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::DatabaseHandle;
use crate::error::{Error, Result};
use crate::sql_types::hashed_password::HashedPassword;
use crate::sql_types::id::UserDataId;
use crate::sql_types::world_data_id::WorldDataId;
use crate::sql_types::zoned::Zoned;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use nil_core::world::{World, WorldId};
use nil_util::password::Password;

#[derive(Identifiable, Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::world_data)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(UserData, foreign_key = created_by))]
pub struct WorldData {
  pub id: WorldDataId,
  pub password: Option<HashedPassword>,
  pub created_by: UserDataId,
  pub created_at: Zoned,
  pub updated_at: Zoned,
  pub data: Vec<u8>,
}

impl WorldData {
  pub fn get(database: &DatabaseHandle, id: WorldId) -> Result<Self> {
    use crate::schema::world_data;

    let id = WorldDataId::from(id);
    let result = world_data::table
      .find(&id)
      .select(Self::as_select())
      .first(&mut *database.conn());

    if let Err(DieselError::NotFound) = &result {
      Err(Error::WorldNotFound(id.into()))
    } else {
      Ok(result?)
    }
  }

  pub fn get_all(database: &DatabaseHandle) -> Result<Vec<Self>> {
    use crate::schema::world_data::dsl::*;
    world_data
      .select(Self::as_select())
      .load(&mut *database.conn())
      .map_err(Into::into)
  }

  pub fn update_data(database: &DatabaseHandle, world: WorldId, data: &[u8]) -> Result<usize> {
    use crate::schema::world_data;
    let id = WorldDataId::from(world);
    diesel::update(world_data::table.filter(world_data::id.eq(&id)))
      .set(world_data::data.eq(data))
      .execute(&mut *database.conn())
      .map_err(Into::into)
  }

  #[inline]
  pub fn into_world(self) -> Result<World> {
    Ok(World::load(&self.data)?)
  }

  #[inline]
  pub fn verify_password(&self, password: &Password) -> bool {
    self
      .password
      .as_ref()
      .is_none_or(|it| it.verify(password))
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
  pub created_by: UserDataId,
  pub created_at: Zoned,
  pub updated_at: Zoned,
}

impl WorldDataless {
  pub fn get(database: &DatabaseHandle, id: WorldId) -> Result<Self> {
    use crate::schema::world_data;

    let id = WorldDataId::from(id);
    let result = world_data::table
      .find(&id)
      .select(Self::as_select())
      .first(&mut *database.conn());

    if let Err(DieselError::NotFound) = &result {
      Err(Error::WorldNotFound(id.into()))
    } else {
      Ok(result?)
    }
  }
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::world_data)]
pub struct NewWorldData {
  id: WorldDataId,
  password: Option<HashedPassword>,
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
    created_by: UserDataId,
  ) -> Result<Self> {
    if let Some(password) = password {
      let pass_len = password.trim().chars().count();
      if !(3..=50).contains(&pass_len) {
        return Err(Error::InvalidPassword);
      }
    }

    Ok(Self {
      id: WorldDataId::from(id),
      password: password
        .map(HashedPassword::new)
        .transpose()?,
      created_by,
      created_at: Zoned::now(),
      updated_at: Zoned::now(),
      data,
    })
  }

  pub fn create(self, database: &DatabaseHandle) -> Result<usize> {
    use crate::schema::world_data::dsl::*;
    diesel::insert_into(world_data)
      .values(&self)
      .on_conflict(id)
      .do_update()
      .set((data.eq(&self.data), updated_at.eq(Zoned::now())))
      .execute(&mut *database.conn())
      .map_err(Into::into)
  }
}
