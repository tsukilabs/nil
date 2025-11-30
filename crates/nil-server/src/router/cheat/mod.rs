// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod city;
mod infrastructure;
mod npc;
mod resources;

pub use city::set_stability;
pub use infrastructure::{set_building_level, set_max_infrastructure};

pub use npc::{
  get_bot_resources,
  get_bot_storage_capacity,
  get_precursor_resources,
  get_precursor_storage_capacity,
  spawn_bot,
};

pub use resources::{
  set_food,
  set_iron,
  set_max_food,
  set_max_iron,
  set_max_resources,
  set_max_silo_resources,
  set_max_stone,
  set_max_warehouse_resources,
  set_max_wood,
  set_resources,
  set_stone,
  set_wood,
};
