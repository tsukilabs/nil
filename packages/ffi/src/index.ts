//! Copyright (C) Call of Nil contributors
//! SPDX-License-Identifier: AGPL-3.0-only

import { Queue } from "./queue";
import * as ffi from "node:ffi";
import { definitions } from "./def";
import { HandleClosedError } from "./error";
import type { Option } from "@tb-dev/utils";
import { version } from "../package.json" with { type: "json" };
import type {
  GetRemoteWorldLimitPerUserResponse,
  GetRemoteWorldLimitResponse,
  GetRemoteWorldsResponse,
  WorldId,
  WorldOptions,
} from "@tsukilabs/nil-bindings";

export * from "./def";
export * from "./error";

export type Handle = ffi.DynamicLibraryResult<typeof definitions>;

// TODO: import from `@tsukilabs/nil-bindings`.
type LocalServer = { world: WorldId; addr: string; };

export const VERSION = version;
export const USER_AGENT = `nil-ffi-node/${VERSION}`;

export class Nil implements Disposable {
  private readonly handle: Handle;
  public readonly functions: Handle["functions"];

  private readonly queue: Queue;
  private disposed = false;

  private constructor(dll: string) {
    if (!dll.endsWith(ffi.suffix)) {
      dll = `${dll}.${ffi.suffix}`;
    }

    this.handle = ffi.dlopen(dll, definitions);
    this.functions = this.handle.functions;
    this.queue = new Queue(this.handle);
  }

  public static async init(dll: string) {
    const nil = new Nil(dll);
    await nil.setUserAgent(USER_AGENT);
    return nil;
  }

  public [Symbol.dispose]() {
    this.close();
  }

  public close() {
    if (this.disposed) {
      return;
    }

    this.disposed = true;
    this.queue[Symbol.dispose]();
    this.functions.nil_ffi_shutdown();
    this.handle[Symbol.dispose]();
  }

  public isClosed() {
    return this.disposed;
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
  public async getUserAgent() {
    return this.queue.request<string>((requestId) => {
      this.functions.nil_user_agent(requestId);
    });
  }

  @ThrowIfClosed
  public async getWorld() {
    return this.queue.request<Option<string>>((requestId) => {
      this.functions.nil_world(requestId);
    });
  }

  @ThrowIfClosed
  public async isLocal() {
    return this.queue.request<boolean>((requestId) => {
      this.functions.nil_is_local(requestId);
    });
  }

  @ThrowIfClosed
  public async setUserAgent(userAgent: string) {
    await this.queue.request((requestId) => {
      const arg1 = JSON.stringify(userAgent);
      this.functions.nil_set_user_agent(requestId, arg1);
    });
  }

  @ThrowIfClosed
  public async startServer(options: WorldOptions) {
    await this.queue.request<LocalServer>((requestId) => {
      const arg1 = JSON.stringify(options);
      this.functions.nil_start_server(requestId, arg1);
    });
  }

  @ThrowIfClosed
  public async startServerWithSavedata(path: string) {
    return this.queue.request<LocalServer>((requestId) => {
      const arg1 = JSON.stringify(path);
      this.functions.nil_start_server_with_savedata(requestId, arg1);
    });
  }

  @ThrowIfClosed
  public async stopServer() {
    await this.queue.request((requestId) => {
      this.functions.nil_stop_server(requestId);
    });
  }
}

function ThrowIfClosed(target: Nil, _key: string, descriptor: PropertyDescriptor) {
  const method = descriptor.value;
  descriptor.value = function(...args: any[]) {
    if (target.isClosed()) {
      throw new HandleClosedError();
    }

    Reflect.apply(method, this, args);
  };
}
