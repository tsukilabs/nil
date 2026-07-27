// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Queue } from "./queue";
import * as ffi from "node:ffi";
import { definitions } from "./def";
import { HandleClosedError } from "./error";
import type { Option } from "@tb-dev/utils";

export * from "./def";
export * from "./error";

export type Handle = ffi.DynamicLibraryResult<typeof definitions>;

export class Nil implements Disposable {
  private readonly handle: Handle;
  public readonly functions: Handle["functions"];

  private readonly queue: Queue;
  private disposed = false;

  constructor(dll: string) {
    if (!dll.endsWith(ffi.suffix)) {
      dll = `${dll}.${ffi.suffix}`;
    }

    this.handle = ffi.dlopen(dll, definitions);
    this.functions = this.handle.functions;
    this.queue = new Queue(this.handle);
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
    this.functions.callofnil_shutdown();
    this.handle[Symbol.dispose]();
  }

  public async getClientVersion() {
    this.throwIfClosed();
    return this.queue.request<string>(() => {
      return this.functions.callofnil_client_version();
    });
  }

  public async getFfiVersion() {
    this.throwIfClosed();
    return this.queue.request<string>(() => {
      return this.functions.callofnil_ffi_version();
    });
  }

  public async getUserAgent() {
    this.throwIfClosed();
    return this.queue.request<string>(() => {
      return this.functions.callofnil_user_agent();
    });
  }

  public async getWorld() {
    this.throwIfClosed();
    return this.queue.request<Option<string>>(() => {
      return this.functions.callofnil_world();
    });
  }

  public async setUserAgent(userAgent: string) {
    this.throwIfClosed();
    await this.queue.request(() => {
      return this.functions.callofnil_set_user_agent(userAgent);
    });
  }

  private throwIfClosed() {
    if (this.disposed) {
      throw new HandleClosedError();
    }
  }
}
