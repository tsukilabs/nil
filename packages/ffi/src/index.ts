// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

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
} from "@tsukilabs/nil-bindings";

export * from "./def";
export * from "./error";

export type Handle = ffi.DynamicLibraryResult<typeof definitions>;

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

    this.setUserAgent(`nil-ffi-node/${version}`)
      .catch(console.error);
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

  public async getClientVersion() {
    this.throwIfClosed();
    return this.queue.request<string>((requestId) => {
      this.functions.nil_client_version(requestId);
    });
  }

  public async getFfiVersion() {
    this.throwIfClosed();
    return this.queue.request<string>((requestId) => {
      this.functions.nil_ffi_version(requestId);
    });
  }

  public async getRemoteWorldLimit() {
    this.throwIfClosed();
    return this.queue.request<GetRemoteWorldLimitResponse>((requestId) => {
      this.functions.nil_get_remote_world_limit(requestId);
    });
  }

  public async getRemoteWorldLimitPerUser() {
    this.throwIfClosed();
    return this.queue.request<GetRemoteWorldLimitPerUserResponse>((requestId) => {
      this.functions.nil_get_remote_world_limit_per_user(requestId);
    });
  }

  public async getRemoteWorlds() {
    this.throwIfClosed();
    return this.queue.request<GetRemoteWorldsResponse>((requestId) => {
      this.functions.nil_get_remote_worlds(requestId);
    });
  }

  public async getUserAgent() {
    this.throwIfClosed();
    return this.queue.request<string>((requestId) => {
      this.functions.nil_user_agent(requestId);
    });
  }

  public async getWorld() {
    this.throwIfClosed();
    return this.queue.request<Option<string>>((requestId) => {
      this.functions.nil_world(requestId);
    });
  }

  public async setUserAgent(userAgent: string) {
    this.throwIfClosed();
    await this.queue.request((requestId) => {
      const arg1 = JSON.stringify(userAgent);
      this.functions.nil_set_user_agent(requestId, arg1);
    });
  }

  private throwIfClosed() {
    if (this.disposed) {
      throw new HandleClosedError();
    }
  }
}
