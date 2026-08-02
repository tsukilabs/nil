//! Copyright (C) Call of Nil contributors
//! SPDX-License-Identifier: AGPL-3.0-only

import { Queue } from "./queue";
import * as ffi from "node:ffi";
import * as path from "node:path";
import { cwd } from "node:process";
import { definitions } from "./def";
import { existsSync } from "node:fs";
import * as fs from "node:fs/promises";
import { HandleClosedError } from "./error";
import type { Option } from "@tb-dev/utils";
import { attemptAsync } from "es-toolkit/util";
import { version } from "../package.json" with { type: "json" };
import type { DownloadLatestOptions, Handle, InitLatestOptions, NilOptions } from "./types";
import type {
  AddAcademyRecruitOrderRequest,
  AddPrefectureBuildOrderRequest,
  AddStableRecruitOrderRequest,
  AddWorkshopRecruitOrderRequest,
  AuthorizeRequest,
  AuthorizeResponse,
  CancelAcademyRecruitOrderRequest,
  CancelManeuverRequest,
  CancelPrefectureBuildOrderRequest,
  CancelStableRecruitOrderRequest,
  CancelWorkshopRecruitOrderRequest,
  CheatFillWorldRequest,
  CheatGetAcademyRecruitQueueRequest,
  CheatGetAcademyRecruitQueueResponse,
  CheatGetAcademyRecruitQueuesRequest,
  CheatGetAcademyRecruitQueuesResponse,
  CheatGetAllAcademyRecruitQueuesRequest,
  CheatGetAllAcademyRecruitQueuesResponse,
  CheatGetAllPrefectureBuildQueuesRequest,
  CheatGetAllPrefectureBuildQueuesResponse,
  CheatGetAllStableRecruitQueuesRequest,
  CheatGetAllStableRecruitQueuesResponse,
  CheatGetBuildStepsRequest,
  CheatGetBuildStepsResponse,
  CheatGetCitiesRequest,
  CheatGetCitiesResponse,
  CheatGetCityRequest,
  CheatGetCityResponse,
  CheatGetEthicsRequest,
  CheatGetEthicsResponse,
  CheatGetIdleArmiesAtRequest,
  CheatGetIdleArmiesAtResponse,
  CheatGetIdlePersonnelAtRequest,
  CheatGetIdlePersonnelAtResponse,
  CheatGetInfrastructureRequest,
  CheatGetInfrastructureResponse,
  CheatGetManeuversOfRequest,
  CheatGetManeuversOfResponse,
  CheatGetManeuversRequest,
  CheatGetManeuversResponse,
  CheatGetPlayerRequest,
  CheatGetPlayerResponse,
  CheatGetPlayersRequest,
  CheatGetPlayersResponse,
  CheatGetPrefectureBuildQueueRequest,
  CheatGetPrefectureBuildQueueResponse,
  CheatGetPrefectureBuildQueuesRequest,
  CheatGetPrefectureBuildQueuesResponse,
  CheatGetResourcesRequest,
  CheatGetResourcesResponse,
  CheatGetStableRecruitQueueRequest,
  CheatGetStableRecruitQueueResponse,
  CheatGetStableRecruitQueuesRequest,
  CheatGetStableRecruitQueuesResponse,
  CheatGetStorageCapacityRequest,
  CheatGetStorageCapacityResponse,
  CheatSetBotEthicsRequest,
  CheatSetBuildingLevelRequest,
  CheatSetFoodRequest,
  CheatSetIronRequest,
  CheatSetMarketFeeRequest,
  CheatSetMaxFoodRequest,
  CheatSetMaxInfrastructureRequest,
  CheatSetMaxIronRequest,
  CheatSetMaxResourcesRequest,
  CheatSetMaxSiloResourcesRequest,
  CheatSetMaxStoneRequest,
  CheatSetMaxWarehouseResourcesRequest,
  CheatSetMaxWoodRequest,
  CheatSetResourcesRequest,
  CheatSetStabilityRequest,
  CheatSetStoneRequest,
  CheatSetWoodRequest,
  CheatSkipRoundRequest,
  CheatSpawnBotRequest,
  CheatSpawnBotResponse,
  CheatSpawnCityRequest,
  CheatSpawnPersonnelRequest,
  ClientOptions,
  CreateRemoteWorldRequest,
  CreateRemoteWorldResponse,
  CreateUserRequest,
  DeleteRemoteWorldRequest,
  ForwardReportRequest,
  GetAcademyRecruitCatalogRequest,
  GetAcademyRecruitCatalogResponse,
  GetArmiesRequest,
  GetArmiesResponse,
  GetArmyOwnerRequest,
  GetArmyOwnerResponse,
  GetArmyRequest,
  GetArmyResponse,
  GetBotCoordsRequest,
  GetBotCoordsResponse,
  GetChatHistoryRequest,
  GetChatHistoryResponse,
  GetCitiesRequest,
  GetCitiesResponse,
  GetCityRequest,
  GetCityResponse,
  GetCityScoreRequest,
  GetCityScoreResponse,
  GetContinentSizeRequest,
  GetContinentSizeResponse,
  GetIdleArmiesAtRequest,
  GetIdleArmiesAtResponse,
  GetIdleArmiesCoordsRequest,
  GetIdleArmiesCoordsResponse,
  GetManeuverRequest,
  GetManeuverResponse,
  GetMarketFeeRequest,
  GetMarketFeeResponse,
  GetPlayerCoordsRequest,
  GetPlayerCoordsResponse,
  GetPlayerIdsRequest,
  GetPlayerIdsResponse,
  GetPlayerMaintenanceRequest,
  GetPlayerMaintenanceResponse,
  GetPlayerMilitaryRequest,
  GetPlayerMilitaryResponse,
  GetPlayerRequest,
  GetPlayerResponse,
  GetPlayerStatusRequest,
  GetPlayerStatusResponse,
  GetPlayerStorageCapacityRequest,
  GetPlayerStorageCapacityResponse,
  GetPlayerWorldsRequest,
  GetPlayerWorldsResponse,
  GetPrecursorCoordsRequest,
  GetPrecursorCoordsResponse,
  GetPrefectureBuildCatalogRequest,
  GetPrefectureBuildCatalogResponse,
  GetPublicBotRequest,
  GetPublicBotResponse,
  GetPublicBotsRequest,
  GetPublicBotsResponse,
  GetPublicCitiesRequest,
  GetPublicCitiesResponse,
  GetPublicCityRequest,
  GetPublicCityResponse,
  GetPublicFieldRequest,
  GetPublicFieldResponse,
  GetPublicFieldsRequest,
  GetPublicFieldsResponse,
  GetPublicPlayerRequest,
  GetPublicPlayerResponse,
  GetPublicPlayersRequest,
  GetPublicPlayersResponse,
  GetPublicPrecursorRequest,
  GetPublicPrecursorResponse,
  GetPublicPrecursorsRequest,
  GetPublicPrecursorsResponse,
  GetRankingRequest,
  GetRankingResponse,
  GetRankRequest,
  GetRankResponse,
  GetRemoteWorldLimitPerUserResponse,
  GetRemoteWorldLimitResponse,
  GetRemoteWorldRequest,
  GetRemoteWorldResponse,
  GetRemoteWorldsResponse,
  GetRoundRequest,
  GetRoundResponse,
  GetServerKindResponse,
  GetStableRecruitCatalogRequest,
  GetStableRecruitCatalogResponse,
  GetWorkshopRecruitCatalogRequest,
  GetWorkshopRecruitCatalogResponse,
  GetWorldBotsRequest,
  GetWorldBotsResponse,
  GetWorldConfigRequest,
  GetWorldConfigResponse,
  GetWorldPersonnelRequest,
  GetWorldPersonnelResponse,
  GetWorldPlayersRequest,
  GetWorldPlayersResponse,
  GetWorldPrecursorsRequest,
  GetWorldPrecursorsResponse,
  GetWorldStatsRequest,
  GetWorldStatsResponse,
  LocalServer,
  PlayerExistsRequest,
  PlayerExistsResponse,
  PushChatMessageRequest,
  PushChatMessageResponse,
  RenameCityRequest,
  RequestManeuverRequest,
  RequestManeuverResponse,
  SaveLocalWorldRequest,
  SearchCityRequest,
  SearchCityResponse,
  SearchPublicCityRequest,
  SearchPublicCityResponse,
  SendResourcesRequest,
  ServerAddr,
  SetPlayerReadyRequest,
  SetPlayerReadyResponse,
  SetPlayerStatusRequest,
  SimulateBattleRequest,
  SimulateBattleResponse,
  SpawnPlayerRequest,
  StartRoundRequest,
  StartRoundResponse,
  ToggleBuildingRequest,
  UserExistsRequest,
  UserExistsResponse,
  ValidateTokenRequest,
  ValidateTokenResponse,
  VersionResponse,
  WorldId,
  WorldOptions,
} from "@tsukilabs/nil-bindings";

export * from "./def";
export * from "./error";

export type * from "./types";

export const VERSION = version;
export const USER_AGENT = `nil-ffi-node/${VERSION}`;

export class Nil implements AsyncDisposable {
  public readonly path: string;
  private readonly handle: Handle;
  private readonly functions: Handle["functions"];

  private readonly queue: Queue;

  private closed = false;
  private closePromise: Option<Promise<void>>;

  private constructor(dll: string, options?: NilOptions) {
    if (!dll.endsWith(ffi.suffix)) {
      dll = `${dll}.${ffi.suffix}`;
    }

    this.path = path.resolve(dll);
    this.handle = ffi.dlopen(dll, definitions);
    this.functions = this.handle.functions;
    this.queue = new Queue(this.handle, options);
  }

  public async [Symbol.asyncDispose]() {
    await this.close();
  }

  async #close() {
    if (this.closed) {
      return;
    }

    this.closed = true;

    await attemptAsync(this.stopClient.bind(this));
    await attemptAsync(this.stopServer.bind(this));

    this.queue[Symbol.dispose]();
    this.functions.nil_ffi_shutdown();
    this.handle[Symbol.dispose]();
  }

  public async close() {
    this.closePromise ??= this.#close()
      .finally(() => void (this.closePromise = null));

    return this.closePromise;
  }

  public isClosed() {
    return this.closed;
  }

  public static async init(dll: string, options?: NilOptions) {
    const nil = new Nil(dll, options);
    await nil.setUserAgent(USER_AGENT);
    return nil;
  }

  public static async initLatest(options?: InitLatestOptions) {
    const dll = await this.downloadLatest(options);
    return this.init(dll, options);
  }

  public static async downloadLatest(options?: DownloadLatestOptions) {
    const file = `libnil.${ffi.suffix}`;
    const filePath = path.resolve(options?.outDir ?? cwd(), file);
    if (!(options?.overwrite ?? true) && existsSync(filePath)) {
      return filePath;
    }

    const response = await fetch(`https://tsukilabs.dev.br/download/nil/${file}`, {
      headers: { "User-Agent": USER_AGENT },
    });

    if (!response.ok) {
      throw new Error(
        `failed to download ${file}: ${response.status} ${response.statusText}`,
      );
    }

    const bytes = await response.bytes();
    await fs.writeFile(filePath, bytes);
    return filePath;
  }

  /////////////////////////////////////////////////////////////////////////////////////////////
  ////////////////////////////////////// FFI FUNCTIONS ////////////////////////////////////////
  /////////////////////////////////////////////////////////////////////////////////////////////

  @ThrowIfClosed
  public async addAcademyRecruitOrder(req: AddAcademyRecruitOrderRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_add_academy_recruit_order(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async addPrefectureBuildOrder(req: AddPrefectureBuildOrderRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_add_prefecture_build_order(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async addStableRecruitOrder(req: AddStableRecruitOrderRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_add_stable_recruit_order(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async addWorkshopRecruitOrder(req: AddWorkshopRecruitOrderRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_add_workshop_recruit_order(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async authorize(req: AuthorizeRequest) {
    return this.queue.request<AuthorizeResponse>((requestId) => {
      this.functions.nil_authorize(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cancelAcademyRecruitOrder(req: CancelAcademyRecruitOrderRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cancel_academy_recruit_order(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cancelManeuver(req: CancelManeuverRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cancel_maneuver(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cancelPrefectureBuildOrder(req: CancelPrefectureBuildOrderRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cancel_prefecture_build_order(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cancelStableRecruitOrder(req: CancelStableRecruitOrderRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cancel_stable_recruit_order(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cancelWorkshopRecruitOrder(req: CancelWorkshopRecruitOrderRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cancel_workshop_recruit_order(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatFillWorld(req: CheatFillWorldRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_fill_world(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetAcademyRecruitQueue(req: CheatGetAcademyRecruitQueueRequest) {
    return this.queue.request<CheatGetAcademyRecruitQueueResponse>((requestId) => {
      this.functions.nil_cheat_get_academy_recruit_queue(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetAcademyRecruitQueues(req: CheatGetAcademyRecruitQueuesRequest) {
    return this.queue.request<CheatGetAcademyRecruitQueuesResponse>((requestId) => {
      this.functions.nil_cheat_get_academy_recruit_queues(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetAllAcademyRecruitQueues(req: CheatGetAllAcademyRecruitQueuesRequest) {
    return this.queue.request<CheatGetAllAcademyRecruitQueuesResponse>((requestId) => {
      this.functions.nil_cheat_get_all_academy_recruit_queues(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetAllPrefectureBuildQueues(req: CheatGetAllPrefectureBuildQueuesRequest) {
    return this.queue.request<CheatGetAllPrefectureBuildQueuesResponse>((requestId) => {
      this.functions.nil_cheat_get_all_prefecture_build_queues(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetAllStableRecruitQueues(req: CheatGetAllStableRecruitQueuesRequest) {
    return this.queue.request<CheatGetAllStableRecruitQueuesResponse>((requestId) => {
      this.functions.nil_cheat_get_all_stable_recruit_queues(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetBuildSteps(req: CheatGetBuildStepsRequest) {
    return this.queue.request<CheatGetBuildStepsResponse>((requestId) => {
      this.functions.nil_cheat_get_build_steps(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetCities(req: CheatGetCitiesRequest) {
    return this.queue.request<CheatGetCitiesResponse>((requestId) => {
      this.functions.nil_cheat_get_cities(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetCity(req: CheatGetCityRequest) {
    return this.queue.request<CheatGetCityResponse>((requestId) => {
      this.functions.nil_cheat_get_city(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetEthics(req: CheatGetEthicsRequest) {
    return this.queue.request<CheatGetEthicsResponse>((requestId) => {
      this.functions.nil_cheat_get_ethics(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetIdleArmiesAt(req: CheatGetIdleArmiesAtRequest) {
    return this.queue.request<CheatGetIdleArmiesAtResponse>((requestId) => {
      this.functions.nil_cheat_get_idle_armies_at(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetIdlePersonnelAt(req: CheatGetIdlePersonnelAtRequest) {
    return this.queue.request<CheatGetIdlePersonnelAtResponse>((requestId) => {
      this.functions.nil_cheat_get_idle_personnel_at(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetInfrastructure(req: CheatGetInfrastructureRequest) {
    return this.queue.request<CheatGetInfrastructureResponse>((requestId) => {
      this.functions.nil_cheat_get_infrastructure(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetManeuvers(req: CheatGetManeuversRequest) {
    return this.queue.request<CheatGetManeuversResponse>((requestId) => {
      this.functions.nil_cheat_get_maneuvers(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetManeuversOf(req: CheatGetManeuversOfRequest) {
    return this.queue.request<CheatGetManeuversOfResponse>((requestId) => {
      this.functions.nil_cheat_get_maneuvers_of(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetPlayer(req: CheatGetPlayerRequest) {
    return this.queue.request<CheatGetPlayerResponse>((requestId) => {
      this.functions.nil_cheat_get_player(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetPlayers(req: CheatGetPlayersRequest) {
    return this.queue.request<CheatGetPlayersResponse>((requestId) => {
      this.functions.nil_cheat_get_players(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetPrefectureBuildQueue(req: CheatGetPrefectureBuildQueueRequest) {
    return this.queue.request<CheatGetPrefectureBuildQueueResponse>((requestId) => {
      this.functions.nil_cheat_get_prefecture_build_queue(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetPrefectureBuildQueues(req: CheatGetPrefectureBuildQueuesRequest) {
    return this.queue.request<CheatGetPrefectureBuildQueuesResponse>((requestId) => {
      this.functions.nil_cheat_get_prefecture_build_queues(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetResources(req: CheatGetResourcesRequest) {
    return this.queue.request<CheatGetResourcesResponse>((requestId) => {
      this.functions.nil_cheat_get_resources(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetStableRecruitQueue(req: CheatGetStableRecruitQueueRequest) {
    return this.queue.request<CheatGetStableRecruitQueueResponse>((requestId) => {
      this.functions.nil_cheat_get_stable_recruit_queue(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetStableRecruitQueues(req: CheatGetStableRecruitQueuesRequest) {
    return this.queue.request<CheatGetStableRecruitQueuesResponse>((requestId) => {
      this.functions.nil_cheat_get_stable_recruit_queues(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatGetStorageCapacity(req: CheatGetStorageCapacityRequest) {
    return this.queue.request<CheatGetStorageCapacityResponse>((requestId) => {
      this.functions.nil_cheat_get_storage_capacity(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSetBotEthics(req: CheatSetBotEthicsRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_set_bot_ethics(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSetBuildingLevel(req: CheatSetBuildingLevelRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_set_building_level(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSetFood(req: CheatSetFoodRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_set_food(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSetIron(req: CheatSetIronRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_set_iron(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSetMarketFee(req: CheatSetMarketFeeRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_set_market_fee(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSetMaxFood(req: CheatSetMaxFoodRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_set_max_food(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSetMaxInfrastructure(req: CheatSetMaxInfrastructureRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_set_max_infrastructure(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSetMaxIron(req: CheatSetMaxIronRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_set_max_iron(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSetMaxResources(req: CheatSetMaxResourcesRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_set_max_resources(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSetMaxSiloResources(req: CheatSetMaxSiloResourcesRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_set_max_silo_resources(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSetMaxStone(req: CheatSetMaxStoneRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_set_max_stone(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSetMaxWarehouseResources(req: CheatSetMaxWarehouseResourcesRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_set_max_warehouse_resources(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSetMaxWood(req: CheatSetMaxWoodRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_set_max_wood(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSetResources(req: CheatSetResourcesRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_set_resources(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSetStability(req: CheatSetStabilityRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_set_stability(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSetStone(req: CheatSetStoneRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_set_stone(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSetWood(req: CheatSetWoodRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_set_wood(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSkipRound(req: CheatSkipRoundRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_skip_round(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSpawnBot(req: CheatSpawnBotRequest) {
    return this.queue.request<CheatSpawnBotResponse>((requestId) => {
      this.functions.nil_cheat_spawn_bot(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSpawnCity(req: CheatSpawnCityRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_spawn_city(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async cheatSpawnPersonnel(req: CheatSpawnPersonnelRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_cheat_spawn_personnel(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async createRemoteWorld(req: CreateRemoteWorldRequest) {
    return this.queue.request<CreateRemoteWorldResponse>((requestId) => {
      this.functions.nil_create_remote_world(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async createUser(req: CreateUserRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_create_user(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async deleteRemoteWorld(req: DeleteRemoteWorldRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_delete_remote_world(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async forwardReport(req: ForwardReportRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_forward_report(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getAcademyRecruitCatalog(req: GetAcademyRecruitCatalogRequest) {
    return this.queue.request<GetAcademyRecruitCatalogResponse>((requestId) => {
      this.functions.nil_get_academy_recruit_catalog(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getArmies(req: GetArmiesRequest) {
    return this.queue.request<GetArmiesResponse>((requestId) => {
      this.functions.nil_get_armies(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getArmy(req: GetArmyRequest) {
    return this.queue.request<GetArmyResponse>((requestId) => {
      this.functions.nil_get_army(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getArmyOwner(req: GetArmyOwnerRequest) {
    return this.queue.request<GetArmyOwnerResponse>((requestId) => {
      this.functions.nil_get_army_owner(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getBotCoords(req: GetBotCoordsRequest) {
    return this.queue.request<GetBotCoordsResponse>((requestId) => {
      this.functions.nil_get_bot_coords(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getChatHistory(req: GetChatHistoryRequest) {
    return this.queue.request<GetChatHistoryResponse>((requestId) => {
      this.functions.nil_get_chat_history(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getCities(req: GetCitiesRequest) {
    return this.queue.request<GetCitiesResponse>((requestId) => {
      this.functions.nil_get_cities(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getCity(req: GetCityRequest) {
    return this.queue.request<GetCityResponse>((requestId) => {
      this.functions.nil_get_city(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getCityScore(req: GetCityScoreRequest) {
    return this.queue.request<GetCityScoreResponse>((requestId) => {
      this.functions.nil_get_city_score(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getClientVersion() {
    return this.queue.request<string>((requestId) => {
      this.functions.nil_client_version(requestId);
    });
  }

  @ThrowIfClosed
  public async getContinentSize(req: GetContinentSizeRequest) {
    return this.queue.request<GetContinentSizeResponse>((requestId) => {
      this.functions.nil_get_continent_size(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getFfiVersion() {
    return this.queue.request<string>((requestId) => {
      this.functions.nil_ffi_version(requestId);
    });
  }

  @ThrowIfClosed
  public async getIdleArmiesAt(req: GetIdleArmiesAtRequest) {
    return this.queue.request<GetIdleArmiesAtResponse>((requestId) => {
      this.functions.nil_get_idle_armies_at(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getIdleArmiesCoords(req: GetIdleArmiesCoordsRequest) {
    return this.queue.request<GetIdleArmiesCoordsResponse>((requestId) => {
      this.functions.nil_get_idle_armies_coords(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getManeuver(req: GetManeuverRequest) {
    return this.queue.request<GetManeuverResponse>((requestId) => {
      this.functions.nil_get_maneuver(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getMarketFee(req: GetMarketFeeRequest) {
    return this.queue.request<GetMarketFeeResponse>((requestId) => {
      this.functions.nil_get_market_fee(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPlayer(req: GetPlayerRequest) {
    return this.queue.request<GetPlayerResponse>((requestId) => {
      this.functions.nil_get_player(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPlayerCoords(req: GetPlayerCoordsRequest) {
    return this.queue.request<GetPlayerCoordsResponse>((requestId) => {
      this.functions.nil_get_player_coords(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPlayerIds(req: GetPlayerIdsRequest) {
    return this.queue.request<GetPlayerIdsResponse>((requestId) => {
      this.functions.nil_get_player_ids(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPlayerMaintenance(req: GetPlayerMaintenanceRequest) {
    return this.queue.request<GetPlayerMaintenanceResponse>((requestId) => {
      this.functions.nil_get_player_maintenance(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPlayerMilitary(req: GetPlayerMilitaryRequest) {
    return this.queue.request<GetPlayerMilitaryResponse>((requestId) => {
      this.functions.nil_get_player_military(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPlayerStatus(req: GetPlayerStatusRequest) {
    return this.queue.request<GetPlayerStatusResponse>((requestId) => {
      this.functions.nil_get_player_status(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPlayerStorageCapacity(req: GetPlayerStorageCapacityRequest) {
    return this.queue.request<GetPlayerStorageCapacityResponse>((requestId) => {
      this.functions.nil_get_player_storage_capacity(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPlayerWorlds(req: GetPlayerWorldsRequest) {
    return this.queue.request<GetPlayerWorldsResponse>((requestId) => {
      this.functions.nil_get_player_worlds(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPrecursorCoords(req: GetPrecursorCoordsRequest) {
    return this.queue.request<GetPrecursorCoordsResponse>((requestId) => {
      this.functions.nil_get_precursor_coords(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPrefectureBuildCatalog(req: GetPrefectureBuildCatalogRequest) {
    return this.queue.request<GetPrefectureBuildCatalogResponse>((requestId) => {
      this.functions.nil_get_prefecture_build_catalog(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPublicBot(req: GetPublicBotRequest) {
    return this.queue.request<GetPublicBotResponse>((requestId) => {
      this.functions.nil_get_public_bot(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPublicBots(req: GetPublicBotsRequest) {
    return this.queue.request<GetPublicBotsResponse>((requestId) => {
      this.functions.nil_get_public_bots(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPublicCities(req: GetPublicCitiesRequest) {
    return this.queue.request<GetPublicCitiesResponse>((requestId) => {
      this.functions.nil_get_public_cities(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPublicCity(req: GetPublicCityRequest) {
    return this.queue.request<GetPublicCityResponse>((requestId) => {
      this.functions.nil_get_public_city(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPublicField(req: GetPublicFieldRequest) {
    return this.queue.request<GetPublicFieldResponse>((requestId) => {
      this.functions.nil_get_public_field(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPublicFields(req: GetPublicFieldsRequest) {
    return this.queue.request<GetPublicFieldsResponse>((requestId) => {
      this.functions.nil_get_public_fields(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPublicPlayer(req: GetPublicPlayerRequest) {
    return this.queue.request<GetPublicPlayerResponse>((requestId) => {
      this.functions.nil_get_public_player(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPublicPlayers(req: GetPublicPlayersRequest) {
    return this.queue.request<GetPublicPlayersResponse>((requestId) => {
      this.functions.nil_get_public_players(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPublicPrecursor(req: GetPublicPrecursorRequest) {
    return this.queue.request<GetPublicPrecursorResponse>((requestId) => {
      this.functions.nil_get_public_precursor(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPublicPrecursors(req: GetPublicPrecursorsRequest) {
    return this.queue.request<GetPublicPrecursorsResponse>((requestId) => {
      this.functions.nil_get_public_precursors(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getRank(req: GetRankRequest) {
    return this.queue.request<GetRankResponse>((requestId) => {
      this.functions.nil_get_rank(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getRanking(req: GetRankingRequest) {
    return this.queue.request<GetRankingResponse>((requestId) => {
      this.functions.nil_get_ranking(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getRemoteWorld(req: GetRemoteWorldRequest) {
    return this.queue.request<GetRemoteWorldResponse>((requestId) => {
      this.functions.nil_get_remote_world(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getRemoteWorldLimit() {
    return this.queue.request<GetRemoteWorldLimitResponse>((requestId) => {
      this.functions.nil_get_remote_world_limit(requestId);
    });
  }

  @ThrowIfClosed
  public async getRemoteWorldLimitPerUser() {
    return this.queue.request<GetRemoteWorldLimitPerUserResponse>((requestId) => {
      this.functions.nil_get_remote_world_limit_per_user(requestId);
    });
  }

  @ThrowIfClosed
  public async getRemoteWorlds() {
    return this.queue.request<GetRemoteWorldsResponse>((requestId) => {
      this.functions.nil_get_remote_worlds(requestId);
    });
  }

  @ThrowIfClosed
  public async getRound(req: GetRoundRequest) {
    return this.queue.request<GetRoundResponse>((requestId) => {
      this.functions.nil_get_round(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getRuntimeGlobalQueueDepth() {
    return this.queue.request<number>((requestId) => {
      this.functions.nil_runtime_global_queue_depth(requestId);
    });
  }

  @ThrowIfClosed
  public async getRuntimeNumAliveTasks() {
    return this.queue.request<number>((requestId) => {
      this.functions.nil_runtime_num_alive_tasks(requestId);
    });
  }

  @ThrowIfClosed
  public async getRuntimeNumWorkers() {
    return this.queue.request<number>((requestId) => {
      this.functions.nil_runtime_num_workers(requestId);
    });
  }

  @ThrowIfClosed
  public async getServerAddr() {
    return this.queue.request<ServerAddr>((requestId) => {
      this.functions.nil_server_addr(requestId);
    });
  }

  @ThrowIfClosed
  public async getServerKind() {
    return this.queue.request<GetServerKindResponse>((requestId) => {
      this.functions.nil_get_server_kind(requestId);
    });
  }

  @ThrowIfClosed
  public async getServerVersion() {
    return this.queue.request<VersionResponse>((requestId) => {
      this.functions.nil_server_version(requestId);
    });
  }

  @ThrowIfClosed
  public async getStableRecruitCatalog(req: GetStableRecruitCatalogRequest) {
    return this.queue.request<GetStableRecruitCatalogResponse>((requestId) => {
      this.functions.nil_get_stable_recruit_catalog(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getUserAgent() {
    return this.queue.request<string>((requestId) => {
      this.functions.nil_user_agent(requestId);
    });
  }

  @ThrowIfClosed
  public async getWorkshopRecruitCatalog(req: GetWorkshopRecruitCatalogRequest) {
    return this.queue.request<GetWorkshopRecruitCatalogResponse>((requestId) => {
      this.functions.nil_get_workshop_recruit_catalog(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getWorldBots(req: GetWorldBotsRequest) {
    return this.queue.request<GetWorldBotsResponse>((requestId) => {
      this.functions.nil_get_world_bots(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getWorldConfig(req: GetWorldConfigRequest) {
    return this.queue.request<GetWorldConfigResponse>((requestId) => {
      this.functions.nil_get_world_config(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getWorldId() {
    return this.queue.request<Option<WorldId>>((requestId) => {
      this.functions.nil_world(requestId);
    });
  }

  @ThrowIfClosed
  public async getWorldPersonnel(req: GetWorldPersonnelRequest) {
    return this.queue.request<GetWorldPersonnelResponse>((requestId) => {
      this.functions.nil_get_world_personnel(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getWorldPlayers(req: GetWorldPlayersRequest) {
    return this.queue.request<GetWorldPlayersResponse>((requestId) => {
      this.functions.nil_get_world_players(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getWorldPrecursors(req: GetWorldPrecursorsRequest) {
    return this.queue.request<GetWorldPrecursorsResponse>((requestId) => {
      this.functions.nil_get_world_precursors(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getWorldStats(req: GetWorldStatsRequest) {
    return this.queue.request<GetWorldStatsResponse>((requestId) => {
      this.functions.nil_get_world_stats(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async isHost() {
    return this.queue.request<boolean>((requestId) => {
      this.functions.nil_is_host(requestId);
    });
  }

  @ThrowIfClosed
  public async isLocal() {
    return this.queue.request<boolean>((requestId) => {
      this.functions.nil_is_local(requestId);
    });
  }

  @ThrowIfClosed
  public async isReady() {
    return this.queue.request<boolean>((requestId) => {
      this.functions.nil_is_ready(requestId);
    });
  }

  @ThrowIfClosed
  public async isRemote() {
    return this.queue.request<boolean>((requestId) => {
      this.functions.nil_is_remote(requestId);
    });
  }

  @ThrowIfClosed
  public async playerExists(req: PlayerExistsRequest) {
    return this.queue.request<PlayerExistsResponse>((requestId) => {
      this.functions.nil_player_exists(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async pushChatMessage(req: PushChatMessageRequest) {
    return this.queue.request<PushChatMessageResponse>((requestId) => {
      this.functions.nil_push_chat_message(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async renameCity(req: RenameCityRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_rename_city(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async requestManeuver(req: RequestManeuverRequest) {
    return this.queue.request<RequestManeuverResponse>((requestId) => {
      this.functions.nil_request_maneuver(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async saveLocalWorld(req: SaveLocalWorldRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_save_local_world(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async searchCity(req: SearchCityRequest) {
    return this.queue.request<SearchCityResponse>((requestId) => {
      this.functions.nil_search_city(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async searchPublicCity(req: SearchPublicCityRequest) {
    return this.queue.request<SearchPublicCityResponse>((requestId) => {
      this.functions.nil_search_public_city(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async sendResources(req: SendResourcesRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_send_resources(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async setPlayerReady(req: SetPlayerReadyRequest) {
    return this.queue.request<SetPlayerReadyResponse>((requestId) => {
      this.functions.nil_set_player_ready(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async setPlayerStatus(req: SetPlayerStatusRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_set_player_status(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async setUserAgent(userAgent: string) {
    return this.queue.request((requestId) => {
      this.functions.nil_set_user_agent(requestId, JSON.stringify(userAgent));
    });
  }

  @ThrowIfClosed
  public async simulateBattle(req: SimulateBattleRequest) {
    return this.queue.request<SimulateBattleResponse>((requestId) => {
      this.functions.nil_simulate_battle(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async spawnPlayer(req: SpawnPlayerRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_spawn_player(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async startRound(req: StartRoundRequest) {
    return this.queue.request<StartRoundResponse>((requestId) => {
      this.functions.nil_start_round(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async startServer(options: WorldOptions) {
    return this.queue.request<LocalServer>((requestId) => {
      this.functions.nil_start_server(requestId, JSON.stringify(options));
    });
  }

  @ThrowIfClosed
  public async startServerWithSavedata(savedataPath: string) {
    return this.queue.request<LocalServer>((requestId) => {
      this.functions.nil_start_server_with_savedata(requestId, JSON.stringify(savedataPath));
    });
  }

  @ThrowIfClosed
  public async stopClient() {
    return this.queue.request((requestId) => {
      this.functions.nil_stop_client(requestId);
    });
  }

  @ThrowIfClosed
  public async stopServer() {
    return this.queue.request((requestId) => {
      this.functions.nil_stop_server(requestId);
    });
  }

  @ThrowIfClosed
  public async toggleBuilding(req: ToggleBuildingRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_toggle_building(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async updateClient(options: ClientOptions) {
    return this.queue.request((requestId) => {
      this.functions.nil_update_client(requestId, JSON.stringify(options));
    });
  }

  @ThrowIfClosed
  public async userExists(req: UserExistsRequest) {
    return this.queue.request<UserExistsResponse>((requestId) => {
      this.functions.nil_user_exists(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async validateToken(req: ValidateTokenRequest) {
    return this.queue.request<ValidateTokenResponse>((requestId) => {
      this.functions.nil_validate_token(requestId, JSON.stringify(req));
    });
  }
}

function ThrowIfClosed(_target: Nil, _key: string, descriptor: PropertyDescriptor) {
  const method = descriptor.value;
  descriptor.value = function(this: Nil, ...args: unknown[]) {
    if (this.isClosed()) {
      throw new HandleClosedError();
    }

    return Reflect.apply(method, this, args);
  };
}
