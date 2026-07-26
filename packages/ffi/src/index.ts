// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as ffi from "node:ffi";
import { FfiError } from "./error";
import { definitions } from "./def";
import * as process from "node:process";
import type { Option } from "@tb-dev/utils";

export * from "./def";
export * from "./error";

export type Handle = ffi.DynamicLibraryResult<typeof definitions>;

export class Nil implements Disposable {
  readonly #handle: Handle;
  #disposed = false;

  public readonly functions: Handle["functions"];

  constructor(dll: string) {
    if (!dll.endsWith(ffi.suffix)) {
      dll = `${dll}.${ffi.suffix}`;
    }

    this.#handle = ffi.dlopen(dll, definitions);
    this.functions = this.#handle.functions;
  }

  public [Symbol.dispose]() {
    this.close();
  }

  public close() {
    this.#disposed = true;
    this.#handle[Symbol.dispose]();
  }

  public getClientVersion() {
    if (this.#disposed) return null;
    const ptr = this.functions.callofnil_client_version();
    return this.ptr.readStr(ptr);
  }

  public getFfiVersion() {
    if (this.#disposed) return null;
    const ptr = this.functions.callofnil_ffi_version();
    return this.ptr.readStr(ptr);
  }

  public getUserAgent() {
    if (this.#disposed) return null;
    const ptr = this.functions.callofnil_user_agent();
    return this.ptr.readStr(ptr);
  }

  public getWorld() {
    if (this.#disposed) return null;
    const ptr = this.functions.callofnil_world();
    return this.ptr.readStr(ptr);
  }

  public setUserAgent(userAgent: string) {
    if (!this.#disposed) {
      const buffer = allocBuffer();
      const status = this.functions.callofnil_set_user_agent(userAgent, buffer);
      if (status !== 0) {
        const err = this.ptr.readStr(buffer.readBigInt64LE());
        throw new FfiError(err, { status });
      }
      else {
        this.ptr.freeStr(buffer.readBigInt64LE());
      }
    }
  }

  public readonly ptr = {
    freeStr: (ptr: Option<bigint>) => {
      if (ptr && !this.#disposed) {
        this.functions.callofnil_free_str(ptr);
      }
    },
    readStr: (ptr: Option<bigint>, free = true) => {
      if (!ptr || this.#disposed) {
        return null;
      }

      try {
        return ffi.toString(ptr);
      }
      finally {
        if (free) {
          this.ptr.freeStr(ptr);
        }
      }
    },
  } as const;
}

export function allocBuffer(size?: number): Buffer {
  size ??= process.arch === "x64" ? 8 : 4;
  return Buffer.alloc(size);
}
