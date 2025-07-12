// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod infrastructure;
mod resource;
mod village;

pub use infrastructure::{set_building_level, set_max_infrastructure};
pub use resource::{
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
pub use village::set_stability;
