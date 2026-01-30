// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Database;
use crate::error::{Error, Result};
use crate::model::world_data::{NewWorldData, WorldData, WorldDataless};
use crate::sql_types::hashed_password::HashedPassword;
use crate::sql_types::world_data_id::WorldDataId;
use crate::sql_types::zoned::Zoned;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use nil_core::world::WorldId;

macro_rules! decl_get {
  ($fn_name:ident, $model:ident) => {
    pub fn $fn_name(&self, id: WorldId) -> Result<$model> {
      use $crate::schema::world_data;

      let id = WorldDataId::from(id);
      let result = world_data::table
        .find(&id)
        .select($model::as_select())
        .first(&mut *self.conn());

      if let Err(DieselError::NotFound) = &result {
        Err(Error::WorldNotFound(id.into()))
      } else {
        Ok(result?)
      }
    }
  };
}

macro_rules! decl_get_all {
  ($fn_name:ident, $model:ident) => {
    pub fn $fn_name(&self) -> Result<Vec<$model>> {
      use crate::schema::world_data::dsl::*;
      world_data
        .select($model::as_select())
        .load(&mut *self.conn())
        .map_err(Into::into)
    }
  };
}

impl Database {
  decl_get!(get_world_data, WorldData);
  decl_get!(get_world_dataless, WorldDataless);
  decl_get_all!(get_worlds_data, WorldData);

  pub fn create_world_data(&self, new: &NewWorldData) -> Result<usize> {
    use crate::schema::world_data::dsl::*;
    diesel::insert_into(world_data)
      .values(new)
      .on_conflict(id)
      .do_update()
      .set((data.eq(new.data()), updated_at.eq(Zoned::now())))
      .execute(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn get_world_password(&self, world_id: WorldId) -> Result<Option<HashedPassword>> {
    use crate::schema::world_data::dsl::*;
    world_data
      .find(&WorldDataId::from(world_id))
      .select(password)
      .first(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn update_world_data(&self, world: WorldId, data: &[u8]) -> Result<usize> {
    use crate::schema::world_data;
    let id = WorldDataId::from(world);
    diesel::update(world_data::table.find(&id))
      .set((
        world_data::data.eq(data),
        world_data::updated_at.eq(Zoned::now()),
      ))
      .execute(&mut *self.conn())
      .map_err(Into::into)
  }
}
