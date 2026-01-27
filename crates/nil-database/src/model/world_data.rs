// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::DatabaseHandle;
use crate::error::{Error, Result};
use crate::sql_types::world_data_id::WorldDataId;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use nil_core::world::{World, WorldId};

#[derive(Identifiable, Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::world_data)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct WorldData {
  pub id: WorldDataId,
  pub data: Vec<u8>,
}

impl WorldData {
  pub fn get(handle: &DatabaseHandle, id: WorldId) -> Result<Self> {
    use crate::schema::world_data;

    let id = WorldDataId::from(id);
    let result = world_data::table
      .filter(world_data::id.eq(&id))
      .select(Self::as_select())
      .first(&mut *handle.conn());

    if let Err(DieselError::NotFound) = &result {
      Err(Error::WorldNotFound(id.into()))
    } else {
      Ok(result?)
    }
  }

  pub fn get_all(handle: &DatabaseHandle) -> Result<Vec<Self>> {
    use crate::schema::world_data::dsl::*;
    world_data
      .select(Self::as_select())
      .load(&mut *handle.conn())
      .map_err(Into::into)
  }

  #[inline]
  pub fn into_world(self) -> Result<World> {
    Ok(World::load(&self.data)?)
  }
}

#[derive(AsChangeset, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::world_data)]
pub struct NewWorldData {
  id: WorldDataId,
  data: Vec<u8>,
}

impl NewWorldData {
  #[inline]
  pub fn new(id: WorldId, data: Vec<u8>) -> Self {
    Self { id: WorldDataId::from(id), data }
  }

  pub fn create(self, handle: &DatabaseHandle) -> Result<usize> {
    use crate::schema::world_data::dsl::*;
    diesel::insert_into(world_data)
      .values(&self)
      .on_conflict(id)
      .do_update()
      .set(&self)
      .execute(&mut *handle.conn())
      .map_err(Into::into)
  }
}
