// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use diesel::prelude::*;

#[derive(Identifiable, Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::world_data)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct WorldData {
  pub id: String,
  pub data: Vec<u8>,
}
