// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod behavior;
mod city;
mod infrastructure;
mod military;
mod npc;
mod player;
mod resources;
mod round;

pub use behavior::get_build_steps;
pub use city::{get_city, set_stability, spawn_city};
pub use infrastructure::{
  get_academy_recruit_queue,
  get_academy_recruit_queues,
  get_all_academy_recruit_queues,
  get_all_prefecture_build_queues,
  get_all_stable_recruit_queues,
  get_infrastructure,
  get_prefecture_build_queue,
  get_prefecture_build_queues,
  get_stable_recruit_queue,
  get_stable_recruit_queues,
  get_storage_capacity,
  set_building_level,
  set_max_infrastructure,
};
pub use military::{
  get_idle_armies_at,
  get_idle_personnel_at,
  get_maneuvers,
  get_maneuvers_of,
  spawn_personnel,
};
pub use npc::{get_ethics, set_bot_ethics, spawn_bot};
pub use player::{get_player, get_players};
pub use resources::{
  get_resources,
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
pub use round::skip_round;

#[doc(hidden)]
#[macro_export]
macro_rules! bail_if_cheats_are_not_allowed {
  ($world:expr) => {
    if !$world.config.are_cheats_allowed() {
      use $crate::error::Error;
      return Err(Error::CheatingNotAllowed);
    }
  };
}
