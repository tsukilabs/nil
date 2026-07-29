// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum Status
#if __STDC_VERSION__ >= 202311L
  : int32_t
#endif // __STDC_VERSION__ >= 202311L
 {
  OK = 0,
  ERR_NULL_POINTER = 1,
  ERR_NOTHING_TO_POLL = 2,
  ERR_SERIALIZATION = 3,
  ERR_UNKNOWN = INT32_MAX,
};
#if __STDC_VERSION__ >= 202311L
typedef enum Status Status;
#else
typedef int32_t Status;
#endif // __STDC_VERSION__ >= 202311L

typedef unsigned int RequestId;

void nil_ffi_free_str(char *ptr);

Status nil_ffi_poll(char **json_out);

void nil_ffi_shutdown(void);

void nil_ffi_version(RequestId request_id);

void nil_is_host(RequestId request_id);

void nil_start_server(RequestId request_id, const char *json_options);

void nil_start_server_with_savedata(RequestId request_id, const char *json_path);

void nil_stop_server(RequestId request_id);

void nil_client_version(RequestId request_id);

void nil_is_local(RequestId request_id);

void nil_is_ready(RequestId request_id);

void nil_is_remote(RequestId request_id);

void nil_server_addr(RequestId request_id);

void nil_set_user_agent(RequestId request_id, const char *json_user_agent);

void nil_stop_client(RequestId request_id);

void nil_update_client(RequestId request_id, const char *json_options);

void nil_user_agent(RequestId request_id);

void nil_world(RequestId request_id);

void nil_runtime_num_alive_tasks(RequestId request_id);

void nil_runtime_num_workers(RequestId request_id);

void nil_add_academy_recruit_order(RequestId request_id, const char *json_req);

void nil_add_prefecture_build_order(RequestId request_id, const char *json_req);

void nil_add_stable_recruit_order(RequestId request_id, const char *json_req);

void nil_add_workshop_recruit_order(RequestId request_id, const char *json_req);

void nil_authorize(RequestId request_id, const char *json_req);

void nil_cancel_academy_recruit_order(RequestId request_id, const char *json_req);

void nil_cancel_maneuver(RequestId request_id, const char *json_req);

void nil_cancel_prefecture_build_order(RequestId request_id, const char *json_req);

void nil_cancel_stable_recruit_order(RequestId request_id, const char *json_req);

void nil_cancel_workshop_recruit_order(RequestId request_id, const char *json_req);

void nil_cheat_fill_world(RequestId request_id, const char *json_req);

void nil_cheat_get_academy_recruit_queue(RequestId request_id, const char *json_req);

void nil_cheat_get_academy_recruit_queues(RequestId request_id, const char *json_req);

void nil_cheat_get_all_academy_recruit_queues(RequestId request_id, const char *json_req);

void nil_cheat_get_all_prefecture_build_queues(RequestId request_id, const char *json_req);

void nil_cheat_get_all_stable_recruit_queues(RequestId request_id, const char *json_req);

void nil_cheat_get_build_steps(RequestId request_id, const char *json_req);

void nil_cheat_get_cities(RequestId request_id, const char *json_req);

void nil_cheat_get_city(RequestId request_id, const char *json_req);

void nil_cheat_get_ethics(RequestId request_id, const char *json_req);

void nil_cheat_get_idle_armies_at(RequestId request_id, const char *json_req);

void nil_cheat_get_idle_personnel_at(RequestId request_id, const char *json_req);

void nil_cheat_get_infrastructure(RequestId request_id, const char *json_req);

void nil_cheat_get_maneuvers(RequestId request_id, const char *json_req);

void nil_cheat_get_maneuvers_of(RequestId request_id, const char *json_req);

void nil_cheat_get_player(RequestId request_id, const char *json_req);

void nil_cheat_get_players(RequestId request_id, const char *json_req);

void nil_cheat_get_prefecture_build_queue(RequestId request_id, const char *json_req);

void nil_cheat_get_prefecture_build_queues(RequestId request_id, const char *json_req);

void nil_cheat_get_resources(RequestId request_id, const char *json_req);

void nil_cheat_get_stable_recruit_queue(RequestId request_id, const char *json_req);

void nil_cheat_get_stable_recruit_queues(RequestId request_id, const char *json_req);

void nil_cheat_get_storage_capacity(RequestId request_id, const char *json_req);

void nil_cheat_set_bot_ethics(RequestId request_id, const char *json_req);

void nil_cheat_set_building_level(RequestId request_id, const char *json_req);

void nil_cheat_set_food(RequestId request_id, const char *json_req);

void nil_cheat_set_iron(RequestId request_id, const char *json_req);

void nil_cheat_set_max_food(RequestId request_id, const char *json_req);

void nil_cheat_set_max_infrastructure(RequestId request_id, const char *json_req);

void nil_cheat_set_max_iron(RequestId request_id, const char *json_req);

void nil_cheat_set_max_resources(RequestId request_id, const char *json_req);

void nil_cheat_set_max_silo_resources(RequestId request_id, const char *json_req);

void nil_cheat_set_max_stone(RequestId request_id, const char *json_req);

void nil_cheat_set_max_warehouse_resources(RequestId request_id, const char *json_req);

void nil_cheat_set_max_wood(RequestId request_id, const char *json_req);

void nil_cheat_set_resources(RequestId request_id, const char *json_req);

void nil_cheat_set_stability(RequestId request_id, const char *json_req);

void nil_cheat_set_stone(RequestId request_id, const char *json_req);

void nil_cheat_set_wood(RequestId request_id, const char *json_req);

void nil_cheat_skip_round(RequestId request_id, const char *json_req);

void nil_cheat_spawn_bot(RequestId request_id, const char *json_req);

void nil_cheat_spawn_city(RequestId request_id, const char *json_req);

void nil_cheat_spawn_personnel(RequestId request_id, const char *json_req);

void nil_create_remote_world(RequestId request_id, const char *json_req);

void nil_create_user(RequestId request_id, const char *json_req);

void nil_delete_remote_world(RequestId request_id, const char *json_req);

void nil_forward_report(RequestId request_id, const char *json_req);

void nil_get_academy_recruit_catalog(RequestId request_id, const char *json_req);

void nil_get_armies(RequestId request_id, const char *json_req);

void nil_get_army(RequestId request_id, const char *json_req);

void nil_get_army_owner(RequestId request_id, const char *json_req);

void nil_get_bot_coords(RequestId request_id, const char *json_req);

void nil_get_chat_history(RequestId request_id, const char *json_req);

void nil_get_cities(RequestId request_id, const char *json_req);

void nil_get_city(RequestId request_id, const char *json_req);

void nil_get_city_score(RequestId request_id, const char *json_req);

void nil_get_continent_size(RequestId request_id, const char *json_req);

void nil_get_idle_armies_at(RequestId request_id, const char *json_req);

void nil_get_idle_armies_coords(RequestId request_id, const char *json_req);

void nil_get_maneuver(RequestId request_id, const char *json_req);

void nil_get_player(RequestId request_id, const char *json_req);

void nil_get_player_coords(RequestId request_id, const char *json_req);

void nil_get_player_ids(RequestId request_id, const char *json_req);

void nil_get_player_maintenance(RequestId request_id, const char *json_req);

void nil_get_player_military(RequestId request_id, const char *json_req);

void nil_get_player_status(RequestId request_id, const char *json_req);

void nil_get_player_storage_capacity(RequestId request_id, const char *json_req);

void nil_get_player_worlds(RequestId request_id, const char *json_req);

void nil_get_precursor_coords(RequestId request_id, const char *json_req);

void nil_get_prefecture_build_catalog(RequestId request_id, const char *json_req);

void nil_get_public_bot(RequestId request_id, const char *json_req);

void nil_get_public_bots(RequestId request_id, const char *json_req);

void nil_get_public_cities(RequestId request_id, const char *json_req);

void nil_get_public_city(RequestId request_id, const char *json_req);

void nil_get_public_field(RequestId request_id, const char *json_req);

void nil_get_public_fields(RequestId request_id, const char *json_req);

void nil_get_public_player(RequestId request_id, const char *json_req);

void nil_get_public_players(RequestId request_id, const char *json_req);

void nil_get_public_precursor(RequestId request_id, const char *json_req);

void nil_get_public_precursors(RequestId request_id, const char *json_req);

void nil_get_rank(RequestId request_id, const char *json_req);

void nil_get_ranking(RequestId request_id, const char *json_req);

void nil_get_remote_world(RequestId request_id, const char *json_req);

void nil_get_remote_world_limit(RequestId request_id);

void nil_get_remote_world_limit_per_user(RequestId request_id);

void nil_get_remote_worlds(RequestId request_id);

void nil_get_round(RequestId request_id, const char *json_req);

void nil_get_server_kind(RequestId request_id);

void nil_get_stable_recruit_catalog(RequestId request_id, const char *json_req);

void nil_get_workshop_recruit_catalog(RequestId request_id, const char *json_req);

void nil_get_world_bots(RequestId request_id, const char *json_req);

void nil_get_world_config(RequestId request_id, const char *json_req);

void nil_get_world_personnel(RequestId request_id, const char *json_req);

void nil_get_world_players(RequestId request_id, const char *json_req);

void nil_get_world_precursors(RequestId request_id, const char *json_req);

void nil_get_world_stats(RequestId request_id, const char *json_req);

void nil_player_exists(RequestId request_id, const char *json_req);

void nil_push_chat_message(RequestId request_id, const char *json_req);

void nil_rename_city(RequestId request_id, const char *json_req);

void nil_request_maneuver(RequestId request_id, const char *json_req);

void nil_save_local_world(RequestId request_id, const char *json_req);

void nil_search_city(RequestId request_id, const char *json_req);

void nil_search_public_city(RequestId request_id, const char *json_req);

void nil_server_version(RequestId request_id);

void nil_set_player_ready(RequestId request_id, const char *json_req);

void nil_set_player_status(RequestId request_id, const char *json_req);

void nil_simulate_battle(RequestId request_id, const char *json_req);

void nil_spawn_player(RequestId request_id, const char *json_req);

void nil_start_round(RequestId request_id, const char *json_req);

void nil_toggle_building(RequestId request_id, const char *json_req);

void nil_user_exists(RequestId request_id, const char *json_req);

void nil_validate_token(RequestId request_id, const char *json_req);
