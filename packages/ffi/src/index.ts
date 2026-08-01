//! Copyright (C) Call of Nil contributors
//! SPDX-License-Identifier: AGPL-3.0-only

import * as ffi from "node:ffi";
import * as path from "node:path";
import { cwd } from "node:process";
import { definitions } from "./def";
import { existsSync } from "node:fs";
import * as fs from "node:fs/promises";
import { HandleClosedError } from "./error";
import type { Option } from "@tb-dev/utils";
import { attemptAsync } from "es-toolkit/util";
import { Queue, type QueueOptions } from "./queue";
import { version } from "../package.json" with { type: "json" };
import type {
  AuthorizeRequest,
  AuthorizeResponse,
  ClientOptions,
  GetPlayerIdsRequest,
  GetPlayerIdsResponse,
  GetPlayerRequest,
  GetPlayerResponse,
  GetPlayerStatusRequest,
  GetPlayerStatusResponse,
  GetRemoteWorldLimitPerUserResponse,
  GetRemoteWorldLimitResponse,
  GetRemoteWorldsResponse,
  GetServerKindResponse,
  LocalServer,
  PlayerExistsRequest,
  PlayerExistsResponse,
  ServerAddr,
  SetPlayerStatusRequest,
  SpawnPlayerRequest,
  ValidateTokenRequest,
  ValidateTokenResponse,
  WorldId,
  WorldOptions,
} from "@tsukilabs/nil-bindings";

export * from "./def";
export * from "./error";

export type Handle = ffi.DynamicLibraryResult<typeof definitions>;
export type { ErrorHandler, QueueOptions } from "./queue";

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
    this.closePromise ??= this.#close();
    return this.closePromise;
  }

  public isClosed() {
    return this.closed;
  }

  @ThrowIfClosed
  public async authorize(req: AuthorizeRequest) {
    return this.queue.request<AuthorizeResponse>((requestId) => {
      this.functions.nil_authorize(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getClientVersion() {
    return this.queue.request<string>((requestId) => {
      this.functions.nil_client_version(requestId);
    });
  }

  @ThrowIfClosed
  public async getFfiVersion() {
    return this.queue.request<string>((requestId) => {
      this.functions.nil_ffi_version(requestId);
    });
  }

  @ThrowIfClosed
  public async getPlayer(req: GetPlayerRequest) {
    return this.queue.request<GetPlayerResponse>((requestId) => {
      this.functions.nil_get_player(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPlayerIds(req: GetPlayerIdsRequest) {
    return this.queue.request<GetPlayerIdsResponse>((requestId) => {
      this.functions.nil_get_player_ids(requestId, JSON.stringify(req));
    });
  }

  @ThrowIfClosed
  public async getPlayerStatus(req: GetPlayerStatusRequest) {
    return this.queue.request<GetPlayerStatusResponse>((requestId) => {
      this.functions.nil_get_player_status(requestId, JSON.stringify(req));
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
  public async getUserAgent() {
    return this.queue.request<string>((requestId) => {
      this.functions.nil_user_agent(requestId);
    });
  }

  @ThrowIfClosed
  public async getWorldId() {
    return this.queue.request<Option<WorldId>>((requestId) => {
      this.functions.nil_world(requestId);
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
  public async spawnPlayer(req: SpawnPlayerRequest) {
    return this.queue.request((requestId) => {
      this.functions.nil_spawn_player(requestId, JSON.stringify(req));
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
  public async updateClient(options: ClientOptions) {
    return this.queue.request((requestId) => {
      this.functions.nil_update_client(requestId, JSON.stringify(options));
    });
  }

  @ThrowIfClosed
  public async validateToken(req: ValidateTokenRequest) {
    return this.queue.request<ValidateTokenResponse>((requestId) => {
      this.functions.nil_validate_token(requestId, JSON.stringify(req));
    });
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

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface NilOptions extends QueueOptions {}

export interface DownloadLatestOptions {
  outDir?: Option<string>;
  overwrite?: boolean;
}

export type InitLatestOptions = DownloadLatestOptions & NilOptions;
