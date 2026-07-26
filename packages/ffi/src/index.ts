// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as ffi from "node:ffi";
import { definitions } from "./def";
import * as process from "node:process";
import type { Option } from "@tb-dev/utils";
import { FfiError, HandleClosedError } from "./error";

export * from "./def";
export * from "./error";

export type Handle = ffi.DynamicLibraryResult<typeof definitions>;

export class Nil implements Disposable {
  private readonly handle: Handle;
  public readonly functions: Handle["functions"];
  private disposed = false;

  constructor(dll: string) {
    if (!dll.endsWith(ffi.suffix)) {
      dll = `${dll}.${ffi.suffix}`;
    }

    this.handle = ffi.dlopen(dll, definitions);
    this.functions = this.handle.functions;
  }

  public [Symbol.dispose]() {
    this.close();
  }

  public close() {
    if (!this.disposed) {
      this.disposed = true;
      this.handle[Symbol.dispose]();
    }
  }

  public getClientVersion() {
    this.throwIfClosed("callofnil_client_version");
    return this.ptr.readStr(
      this.functions.callofnil_client_version(),
    );
  }

  public getFfiVersion() {
    this.throwIfClosed("callofnil_ffi_version");
    return this.ptr.readStr(
      this.functions.callofnil_ffi_version(),
    );
  }

  public getUserAgent() {
    this.throwIfClosed("callofnil_user_agent");
    return this.ptr.readStr(
      this.functions.callofnil_user_agent(),
    );
  }

  public getWorld() {
    this.throwIfClosed("callofnil_world");
    return this.ptr.readStr(
      this.functions.callofnil_world(),
    );
  }

  public setUserAgent(userAgent: string) {
    this.throwIfClosed("callofnil_set_user_agent");
    const out = allocBuffer();
    const status = this.functions.callofnil_set_user_agent(userAgent, out);
    const ptr = out.readBigUInt64LE();

    if (status !== 0) {
      const err = this.ptr.readStr(ptr);
      throw new FfiError(err, { status });
    }
    else {
      this.ptr.freeStr(ptr);
    }
  }

  private throwIfClosed(operation?: Option<string>) {
    if (this.disposed) {
      throw new HandleClosedError(operation);
    }
  }

  public readonly ptr = {
    freeStr: (ptr: Option<bigint>) => {
      this.throwIfClosed("callofnil_free_str");
      if (ptr) this.functions.callofnil_free_str(ptr);
    },
    readStr: (ptr: Option<bigint>, free = true) => {
      this.throwIfClosed();
      if (!ptr) {
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
  return Buffer.alloc(size ?? pointerSize());
}

function pointerSize() {
  if (process.arch === "x64" || process.arch === "arm64") {
    return 8;
  }
  else {
    return 4;
  }
}
