// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_payload::request::prelude::*;
use nil_payload::response::prelude::*;

pub trait Request {
  type Response;

  async fn send(self, client: &Client) -> Result<Self::Response>;
}

macro_rules! impl_request {
  (function = $fun:ident, request = $req:ty, response = $res:ty) => {
    impl Request for $req {
      type Response = $res;

      async fn send(self, client: &Client) -> Result<Self::Response> {
        client.$fun(self).await
      }
    }
  };
}

macro_rules! dispatch {
  ($($name:ident),+) => {
    paste::paste! {
      $(
        impl_request!(
          function = $name,
          request = [<$name:camel Request>],
          response = [<$name:camel Response>]
        );
      )+
    }
  };
}

macro_rules! dispatch_unit {
  ($($name:ident),+) => {
    paste::paste! {
      $(
        impl_request!(
          function = $name,
          request = [<$name:camel Request>],
          response = ()
        );
      )+
    }
  };
}

dispatch!(
  authorize,
  cheat_get_academy_recruit_queue,
  cheat_get_academy_recruit_queues,
  cheat_get_all_academy_recruit_queues,
  cheat_get_all_prefecture_build_queues,
  cheat_get_all_stable_recruit_queues,
  cheat_get_build_steps,
  cheat_get_ethics,
  cheat_get_idle_armies_at,
  cheat_get_idle_personnel_at,
  cheat_get_infrastructure,
  cheat_get_maneuvers,
  cheat_get_maneuvers_of,
  cheat_get_player,
  cheat_get_players,
  cheat_get_prefecture_build_queue,
  cheat_get_prefecture_build_queues,
  cheat_get_resources,
  cheat_get_stable_recruit_queue,
  cheat_get_stable_recruit_queues,
  cheat_get_storage_capacity,
  cheat_spawn_bot,
  create_remote_world,
  get_academy_recruit_catalog,
  get_bot_coords,
  get_chat_history,
  get_city,
  get_city_score,
  get_continent_size,
  get_player,
  get_player_coords,
  get_player_ids,
  get_player_maintenance,
  get_player_military,
  get_player_reports,
  get_player_status,
  get_player_storage_capacity,
  get_player_worlds,
  get_precursor_coords,
  get_prefecture_build_catalog,
  get_public_bot,
  get_public_bots,
  get_public_cities,
  get_public_city,
  get_public_field,
  get_public_fields,
  get_public_player,
  get_public_players,
  get_public_precursor,
  get_public_precursors,
  get_rank,
  get_ranking,
  get_remote_world,
  get_report,
  get_reports,
  get_round,
  get_stable_recruit_catalog,
  get_workshop_recruit_catalog,
  get_world_bots,
  get_world_config,
  get_world_personnel,
  get_world_players,
  get_world_precursors,
  get_world_stats,
  player_exists,
  push_chat_message,
  request_maneuver,
  search_city,
  search_public_city,
  set_player_ready,
  simulate_battle,
  start_round,
  user_exists,
  validate_token
);

dispatch_unit!(
  add_academy_recruit_order,
  add_prefecture_build_order,
  add_stable_recruit_order,
  add_workshop_recruit_order,
  cancel_academy_recruit_order,
  cancel_prefecture_build_order,
  cancel_stable_recruit_order,
  cancel_workshop_recruit_order,
  cheat_skip_round,
  cheat_set_bot_ethics,
  cheat_set_building_level,
  cheat_set_food,
  cheat_set_iron,
  cheat_set_max_food,
  cheat_set_max_infrastructure,
  cheat_set_max_iron,
  cheat_set_max_resources,
  cheat_set_max_silo_resources,
  cheat_set_max_stone,
  cheat_set_max_warehouse_resources,
  cheat_set_max_wood,
  cheat_set_resources,
  cheat_set_stability,
  cheat_set_stone,
  cheat_set_wood,
  cheat_spawn_personnel,
  delete_remote_world,
  create_user,
  forward_report,
  remove_report,
  rename_city,
  save_local_world,
  set_player_status,
  spawn_player,
  toggle_building
);
