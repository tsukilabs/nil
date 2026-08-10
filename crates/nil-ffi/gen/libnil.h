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

/**
 * [`nil_server::local::start`]
 */
void nil_start_server(RequestId request_id, const char *json_options);

/**
 * [`nil_server::local::load`]
 */
void nil_start_server_with_savedata(RequestId request_id, const char *json_path);

/**
 * [`LocalServer::stop`](nil_server::local::LocalServer::stop)
 */
void nil_stop_server(RequestId request_id);

void nil_client_version(RequestId request_id);

/**
 * [`Client::is_local`](nil_client::Client::is_local)
 */
void nil_is_local(RequestId request_id);

/**
 * [`Client::is_ready`](nil_client::Client::is_ready)
 */
void nil_is_ready(RequestId request_id);

/**
 * [`Client::is_remote`](nil_client::Client::is_remote)
 */
void nil_is_remote(RequestId request_id);

/**
 * [`Client::server_addr`](nil_client::Client::server_addr)
 */
void nil_server_addr(RequestId request_id);

/**
 * [`Client::set_user_agent`](nil_client::Client::set_user_agent)
 */
void nil_set_user_agent(RequestId request_id, const char *json_user_agent);

/**
 * [`Client::stop`](nil_client::Client::stop)
 */
void nil_stop_client(RequestId request_id);

/**
 * [`Client::update`](nil_client::Client::update)
 */
void nil_update_client(RequestId request_id, const char *json_options);

/**
 * [`Client::user_agent`](nil_client::Client::user_agent)
 */
void nil_user_agent(RequestId request_id);

/**
 * [`Client::world`](nil_client::Client::world)
 */
void nil_world(RequestId request_id);

/**
 * [`RuntimeMetrics::global_queue_depth`](tokio::runtime::RuntimeMetrics::global_queue_depth)
 */
void nil_runtime_global_queue_depth(RequestId request_id);

/**
 * [`RuntimeMetrics::num_alive_tasks`](tokio::runtime::RuntimeMetrics::num_alive_tasks)
 */
void nil_runtime_num_alive_tasks(RequestId request_id);

/**
 * [`RuntimeMetrics::num_workers`](tokio::runtime::RuntimeMetrics::num_workers)
 */
void nil_runtime_num_workers(RequestId request_id);

/**
 * [`Client::add_academy_recruit_order`](nil_client::Client::add_academy_recruit_order)
 */
void nil_add_academy_recruit_order(RequestId request_id, const char *json_req);

/**
 * [`Client::add_prefecture_build_order`](nil_client::Client::add_prefecture_build_order)
 */
void nil_add_prefecture_build_order(RequestId request_id, const char *json_req);

/**
 * [`Client::add_stable_recruit_order`](nil_client::Client::add_stable_recruit_order)
 */
void nil_add_stable_recruit_order(RequestId request_id, const char *json_req);

/**
 * [`Client::add_workshop_recruit_order`](nil_client::Client::add_workshop_recruit_order)
 */
void nil_add_workshop_recruit_order(RequestId request_id, const char *json_req);

/**
 * [`Client::authorize`](nil_client::Client::authorize)
 */
void nil_authorize(RequestId request_id, const char *json_req);

/**
 * [`Client::buy_resources`](nil_client::Client::buy_resources)
 */
void nil_buy_resources(RequestId request_id, const char *json_req);

/**
 * [`Client::cancel_academy_recruit_order`](nil_client::Client::cancel_academy_recruit_order)
 */
void nil_cancel_academy_recruit_order(RequestId request_id, const char *json_req);

/**
 * [`Client::cancel_maneuver`](nil_client::Client::cancel_maneuver)
 */
void nil_cancel_maneuver(RequestId request_id, const char *json_req);

/**
 * [`Client::cancel_prefecture_build_order`](nil_client::Client::cancel_prefecture_build_order)
 */
void nil_cancel_prefecture_build_order(RequestId request_id, const char *json_req);

/**
 * [`Client::cancel_stable_recruit_order`](nil_client::Client::cancel_stable_recruit_order)
 */
void nil_cancel_stable_recruit_order(RequestId request_id, const char *json_req);

/**
 * [`Client::cancel_workshop_recruit_order`](nil_client::Client::cancel_workshop_recruit_order)
 */
void nil_cancel_workshop_recruit_order(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_fill_world`](nil_client::Client::cheat_fill_world)
 */
void nil_cheat_fill_world(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_get_academy_recruit_queue`](nil_client::Client::cheat_get_academy_recruit_queue)
 */
void nil_cheat_get_academy_recruit_queue(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_get_academy_recruit_queues`](nil_client::Client::cheat_get_academy_recruit_queues)
 */
void nil_cheat_get_academy_recruit_queues(RequestId request_id,
                                          const char *json_req);

/**
 * [`Client::cheat_get_all_academy_recruit_queues`](nil_client::Client::cheat_get_all_academy_recruit_queues)
 */
void nil_cheat_get_all_academy_recruit_queues(RequestId request_id,
                                              const char *json_req);

/**
 * [`Client::cheat_get_all_prefecture_build_queues`](nil_client::Client::cheat_get_all_prefecture_build_queues)
 */
void nil_cheat_get_all_prefecture_build_queues(RequestId request_id,
                                               const char *json_req);

/**
 * [`Client::cheat_get_all_stable_recruit_queues`](nil_client::Client::cheat_get_all_stable_recruit_queues)
 */
void nil_cheat_get_all_stable_recruit_queues(RequestId request_id,
                                             const char *json_req);

/**
 * [`Client::cheat_get_build_steps`](nil_client::Client::cheat_get_build_steps)
 */
void nil_cheat_get_build_steps(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_get_cities`](nil_client::Client::cheat_get_cities)
 */
void nil_cheat_get_cities(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_get_city`](nil_client::Client::cheat_get_city)
 */
void nil_cheat_get_city(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_get_ethics`](nil_client::Client::cheat_get_ethics)
 */
void nil_cheat_get_ethics(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_get_idle_armies_at`](nil_client::Client::cheat_get_idle_armies_at)
 */
void nil_cheat_get_idle_armies_at(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_get_idle_personnel_at`](nil_client::Client::cheat_get_idle_personnel_at)
 */
void nil_cheat_get_idle_personnel_at(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_get_influence`](nil_client::Client::cheat_get_influence)
 */
void nil_cheat_get_influence(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_get_infrastructure`](nil_client::Client::cheat_get_infrastructure)
 */
void nil_cheat_get_infrastructure(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_get_maneuvers`](nil_client::Client::cheat_get_maneuvers)
 */
void nil_cheat_get_maneuvers(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_get_maneuvers_of`](nil_client::Client::cheat_get_maneuvers_of)
 */
void nil_cheat_get_maneuvers_of(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_get_player`](nil_client::Client::cheat_get_player)
 */
void nil_cheat_get_player(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_get_players`](nil_client::Client::cheat_get_players)
 */
void nil_cheat_get_players(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_get_prefecture_build_queue`](nil_client::Client::cheat_get_prefecture_build_queue)
 */
void nil_cheat_get_prefecture_build_queue(RequestId request_id,
                                          const char *json_req);

/**
 * [`Client::cheat_get_prefecture_build_queues`](nil_client::Client::cheat_get_prefecture_build_queues)
 */
void nil_cheat_get_prefecture_build_queues(RequestId request_id,
                                           const char *json_req);

/**
 * [`Client::cheat_get_resources`](nil_client::Client::cheat_get_resources)
 */
void nil_cheat_get_resources(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_get_stable_recruit_queue`](nil_client::Client::cheat_get_stable_recruit_queue)
 */
void nil_cheat_get_stable_recruit_queue(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_get_stable_recruit_queues`](nil_client::Client::cheat_get_stable_recruit_queues)
 */
void nil_cheat_get_stable_recruit_queues(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_get_storage_capacity`](nil_client::Client::cheat_get_storage_capacity)
 */
void nil_cheat_get_storage_capacity(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_set_bot_ethics`](nil_client::Client::cheat_set_bot_ethics)
 */
void nil_cheat_set_bot_ethics(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_set_building_level`](nil_client::Client::cheat_set_building_level)
 */
void nil_cheat_set_building_level(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_set_food`](nil_client::Client::cheat_set_food)
 */
void nil_cheat_set_food(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_set_influence`](nil_client::Client::cheat_set_influence)
 */
void nil_cheat_set_influence(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_set_iron`](nil_client::Client::cheat_set_iron)
 */
void nil_cheat_set_iron(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_set_market_fee`](nil_client::Client::cheat_set_market_fee)
 */
void nil_cheat_set_market_fee(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_set_market_vault_resources`](nil_client::Client::cheat_set_market_vault_resources)
 */
void nil_cheat_set_market_vault_resources(RequestId request_id,
                                          const char *json_req);

/**
 * [`Client::cheat_set_max_food`](nil_client::Client::cheat_set_max_food)
 */
void nil_cheat_set_max_food(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_set_max_infrastructure`](nil_client::Client::cheat_set_max_infrastructure)
 */
void nil_cheat_set_max_infrastructure(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_set_max_iron`](nil_client::Client::cheat_set_max_iron)
 */
void nil_cheat_set_max_iron(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_set_max_resources`](nil_client::Client::cheat_set_max_resources)
 */
void nil_cheat_set_max_resources(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_set_max_silo_resources`](nil_client::Client::cheat_set_max_silo_resources)
 */
void nil_cheat_set_max_silo_resources(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_set_max_stone`](nil_client::Client::cheat_set_max_stone)
 */
void nil_cheat_set_max_stone(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_set_max_warehouse_resources`](nil_client::Client::cheat_set_max_warehouse_resources)
 */
void nil_cheat_set_max_warehouse_resources(RequestId request_id,
                                           const char *json_req);

/**
 * [`Client::cheat_set_max_wood`](nil_client::Client::cheat_set_max_wood)
 */
void nil_cheat_set_max_wood(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_set_resources`](nil_client::Client::cheat_set_resources)
 */
void nil_cheat_set_resources(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_set_stability`](nil_client::Client::cheat_set_stability)
 */
void nil_cheat_set_stability(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_set_stone`](nil_client::Client::cheat_set_stone)
 */
void nil_cheat_set_stone(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_set_wood`](nil_client::Client::cheat_set_wood)
 */
void nil_cheat_set_wood(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_skip_round`](nil_client::Client::cheat_skip_round)
 */
void nil_cheat_skip_round(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_spawn_bot`](nil_client::Client::cheat_spawn_bot)
 */
void nil_cheat_spawn_bot(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_spawn_city`](nil_client::Client::cheat_spawn_city)
 */
void nil_cheat_spawn_city(RequestId request_id, const char *json_req);

/**
 * [`Client::cheat_spawn_personnel`](nil_client::Client::cheat_spawn_personnel)
 */
void nil_cheat_spawn_personnel(RequestId request_id, const char *json_req);

/**
 * [`Client::create_remote_world`](nil_client::Client::create_remote_world)
 */
void nil_create_remote_world(RequestId request_id, const char *json_req);

/**
 * [`Client::create_user`](nil_client::Client::create_user)
 */
void nil_create_user(RequestId request_id, const char *json_req);

/**
 * [`Client::delete_remote_world`](nil_client::Client::delete_remote_world)
 */
void nil_delete_remote_world(RequestId request_id, const char *json_req);

/**
 * [`Client::forward_report`](nil_client::Client::forward_report)
 */
void nil_forward_report(RequestId request_id, const char *json_req);

/**
 * [`Client::get_academy_recruit_catalog`](nil_client::Client::get_academy_recruit_catalog)
 */
void nil_get_academy_recruit_catalog(RequestId request_id, const char *json_req);

/**
 * [`Client::get_armies`](nil_client::Client::get_armies)
 */
void nil_get_armies(RequestId request_id, const char *json_req);

/**
 * [`Client::get_army`](nil_client::Client::get_army)
 */
void nil_get_army(RequestId request_id, const char *json_req);

/**
 * [`Client::get_army_owner`](nil_client::Client::get_army_owner)
 */
void nil_get_army_owner(RequestId request_id, const char *json_req);

/**
 * [`Client::get_bot_coords`](nil_client::Client::get_bot_coords)
 */
void nil_get_bot_coords(RequestId request_id, const char *json_req);

/**
 * [`Client::get_chat_history`](nil_client::Client::get_chat_history)
 */
void nil_get_chat_history(RequestId request_id, const char *json_req);

/**
 * [`Client::get_cities`](nil_client::Client::get_cities)
 */
void nil_get_cities(RequestId request_id, const char *json_req);

/**
 * [`Client::get_city`](nil_client::Client::get_city)
 */
void nil_get_city(RequestId request_id, const char *json_req);

/**
 * [`Client::get_city_limit`](nil_client::Client::get_city_limit)
 */
void nil_get_city_limit(RequestId request_id, const char *json_req);

/**
 * [`Client::get_city_score`](nil_client::Client::get_city_score)
 */
void nil_get_city_score(RequestId request_id, const char *json_req);

/**
 * [`Client::get_continent_size`](nil_client::Client::get_continent_size)
 */
void nil_get_continent_size(RequestId request_id, const char *json_req);

/**
 * [`Client::get_idle_armies_at`](nil_client::Client::get_idle_armies_at)
 */
void nil_get_idle_armies_at(RequestId request_id, const char *json_req);

/**
 * [`Client::get_idle_armies_coords`](nil_client::Client::get_idle_armies_coords)
 */
void nil_get_idle_armies_coords(RequestId request_id, const char *json_req);

/**
 * [`Client::get_maneuver`](nil_client::Client::get_maneuver)
 */
void nil_get_maneuver(RequestId request_id, const char *json_req);

/**
 * [`Client::get_market`](nil_client::Client::get_market)
 */
void nil_get_market(RequestId request_id, const char *json_req);

/**
 * [`Client::get_market_fee`](nil_client::Client::get_market_fee)
 */
void nil_get_market_fee(RequestId request_id, const char *json_req);

/**
 * [`Client::get_player`](nil_client::Client::get_player)
 */
void nil_get_player(RequestId request_id, const char *json_req);

/**
 * [`Client::get_player_coords`](nil_client::Client::get_player_coords)
 */
void nil_get_player_coords(RequestId request_id, const char *json_req);

/**
 * [`Client::get_player_ids`](nil_client::Client::get_player_ids)
 */
void nil_get_player_ids(RequestId request_id, const char *json_req);

/**
 * [`Client::get_player_maintenance`](nil_client::Client::get_player_maintenance)
 */
void nil_get_player_maintenance(RequestId request_id, const char *json_req);

/**
 * [`Client::get_player_military`](nil_client::Client::get_player_military)
 */
void nil_get_player_military(RequestId request_id, const char *json_req);

/**
 * [`Client::get_player_status`](nil_client::Client::get_player_status)
 */
void nil_get_player_status(RequestId request_id, const char *json_req);

/**
 * [`Client::get_player_storage_capacity`](nil_client::Client::get_player_storage_capacity)
 */
void nil_get_player_storage_capacity(RequestId request_id, const char *json_req);

/**
 * [`Client::get_player_worlds`](nil_client::Client::get_player_worlds)
 */
void nil_get_player_worlds(RequestId request_id, const char *json_req);

/**
 * [`Client::get_precursor_coords`](nil_client::Client::get_precursor_coords)
 */
void nil_get_precursor_coords(RequestId request_id, const char *json_req);

/**
 * [`Client::get_prefecture_build_catalog`](nil_client::Client::get_prefecture_build_catalog)
 */
void nil_get_prefecture_build_catalog(RequestId request_id, const char *json_req);

/**
 * [`Client::get_public_bot`](nil_client::Client::get_public_bot)
 */
void nil_get_public_bot(RequestId request_id, const char *json_req);

/**
 * [`Client::get_public_bots`](nil_client::Client::get_public_bots)
 */
void nil_get_public_bots(RequestId request_id, const char *json_req);

/**
 * [`Client::get_public_cities`](nil_client::Client::get_public_cities)
 */
void nil_get_public_cities(RequestId request_id, const char *json_req);

/**
 * [`Client::get_public_city`](nil_client::Client::get_public_city)
 */
void nil_get_public_city(RequestId request_id, const char *json_req);

/**
 * [`Client::get_public_field`](nil_client::Client::get_public_field)
 */
void nil_get_public_field(RequestId request_id, const char *json_req);

/**
 * [`Client::get_public_fields`](nil_client::Client::get_public_fields)
 */
void nil_get_public_fields(RequestId request_id, const char *json_req);

/**
 * [`Client::get_public_player`](nil_client::Client::get_public_player)
 */
void nil_get_public_player(RequestId request_id, const char *json_req);

/**
 * [`Client::get_public_players`](nil_client::Client::get_public_players)
 */
void nil_get_public_players(RequestId request_id, const char *json_req);

/**
 * [`Client::get_public_precursor`](nil_client::Client::get_public_precursor)
 */
void nil_get_public_precursor(RequestId request_id, const char *json_req);

/**
 * [`Client::get_public_precursors`](nil_client::Client::get_public_precursors)
 */
void nil_get_public_precursors(RequestId request_id, const char *json_req);

/**
 * [`Client::get_rank`](nil_client::Client::get_rank)
 */
void nil_get_rank(RequestId request_id, const char *json_req);

/**
 * [`Client::get_ranking`](nil_client::Client::get_ranking)
 */
void nil_get_ranking(RequestId request_id, const char *json_req);

/**
 * [`Client::get_remote_world`](nil_client::Client::get_remote_world)
 */
void nil_get_remote_world(RequestId request_id, const char *json_req);

/**
 * [`Client::get_remote_world_limit`](nil_client::Client::get_remote_world_limit)
 */
void nil_get_remote_world_limit(RequestId request_id);

/**
 * [`Client::get_remote_world_limit_per_user`](nil_client::Client::get_remote_world_limit_per_user)
 */
void nil_get_remote_world_limit_per_user(RequestId request_id);

/**
 * [`Client::get_remote_worlds`](nil_client::Client::get_remote_worlds)
 */
void nil_get_remote_worlds(RequestId request_id);

/**
 * [`Client::get_round`](nil_client::Client::get_round)
 */
void nil_get_round(RequestId request_id, const char *json_req);

/**
 * [`Client::get_server_kind`](nil_client::Client::get_server_kind)
 */
void nil_get_server_kind(RequestId request_id);

/**
 * [`Client::get_stable_recruit_catalog`](nil_client::Client::get_stable_recruit_catalog)
 */
void nil_get_stable_recruit_catalog(RequestId request_id, const char *json_req);

/**
 * [`Client::get_workshop_recruit_catalog`](nil_client::Client::get_workshop_recruit_catalog)
 */
void nil_get_workshop_recruit_catalog(RequestId request_id, const char *json_req);

/**
 * [`Client::get_world_bots`](nil_client::Client::get_world_bots)
 */
void nil_get_world_bots(RequestId request_id, const char *json_req);

/**
 * [`Client::get_world_config`](nil_client::Client::get_world_config)
 */
void nil_get_world_config(RequestId request_id, const char *json_req);

/**
 * [`Client::get_world_personnel`](nil_client::Client::get_world_personnel)
 */
void nil_get_world_personnel(RequestId request_id, const char *json_req);

/**
 * [`Client::get_world_players`](nil_client::Client::get_world_players)
 */
void nil_get_world_players(RequestId request_id, const char *json_req);

/**
 * [`Client::get_world_precursors`](nil_client::Client::get_world_precursors)
 */
void nil_get_world_precursors(RequestId request_id, const char *json_req);

/**
 * [`Client::get_world_rulers`](nil_client::Client::get_world_rulers)
 */
void nil_get_world_rulers(RequestId request_id, const char *json_req);

/**
 * [`Client::get_world_stats`](nil_client::Client::get_world_stats)
 */
void nil_get_world_stats(RequestId request_id, const char *json_req);

/**
 * [`Client::player_exists`](nil_client::Client::player_exists)
 */
void nil_player_exists(RequestId request_id, const char *json_req);

/**
 * [`Client::push_chat_message`](nil_client::Client::push_chat_message)
 */
void nil_push_chat_message(RequestId request_id, const char *json_req);

/**
 * [`Client::rename_city`](nil_client::Client::rename_city)
 */
void nil_rename_city(RequestId request_id, const char *json_req);

/**
 * [`Client::request_maneuver`](nil_client::Client::request_maneuver)
 */
void nil_request_maneuver(RequestId request_id, const char *json_req);

/**
 * [`Client::save_local_world`](nil_client::Client::save_local_world)
 */
void nil_save_local_world(RequestId request_id, const char *json_req);

/**
 * [`Client::search_city`](nil_client::Client::search_city)
 */
void nil_search_city(RequestId request_id, const char *json_req);

/**
 * [`Client::search_public_city`](nil_client::Client::search_public_city)
 */
void nil_search_public_city(RequestId request_id, const char *json_req);

/**
 * [`Client::sell_resources`](nil_client::Client::sell_resources)
 */
void nil_sell_resources(RequestId request_id, const char *json_req);

/**
 * [`Client::send_resources`](nil_client::Client::send_resources)
 */
void nil_send_resources(RequestId request_id, const char *json_req);

/**
 * [`Client::version`](nil_client::Client::version)
 */
void nil_server_version(RequestId request_id);

/**
 * [`Client::set_player_ready`](nil_client::Client::set_player_ready)
 */
void nil_set_player_ready(RequestId request_id, const char *json_req);

/**
 * [`Client::set_player_status`](nil_client::Client::set_player_status)
 */
void nil_set_player_status(RequestId request_id, const char *json_req);

/**
 * [`Client::simulate_battle`](nil_client::Client::simulate_battle)
 */
void nil_simulate_battle(RequestId request_id, const char *json_req);

/**
 * [`Client::spawn_player`](nil_client::Client::spawn_player)
 */
void nil_spawn_player(RequestId request_id, const char *json_req);

/**
 * [`Client::start_round`](nil_client::Client::start_round)
 */
void nil_start_round(RequestId request_id, const char *json_req);

/**
 * [`Client::toggle_building`](nil_client::Client::toggle_building)
 */
void nil_toggle_building(RequestId request_id, const char *json_req);

/**
 * [`Client::user_exists`](nil_client::Client::user_exists)
 */
void nil_user_exists(RequestId request_id, const char *json_req);

/**
 * [`Client::validate_token`](nil_client::Client::validate_token)
 */
void nil_validate_token(RequestId request_id, const char *json_req);
