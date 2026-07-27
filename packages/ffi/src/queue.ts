// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Handle } from ".";
import * as ffi from "node:ffi";
import type { Option } from "@tb-dev/utils";
import { isNil } from "es-toolkit/predicate";
import { allocBuffer, readBufferPtr } from "./ptr";
import { FfiError, HandleClosedError } from "./error";
import {
  type ffi_QueueEntry,
  type ffi_RequestId,
  type ffi_Response,
  ffi_Status,
} from "@tsukilabs/nil-bindings";

export class Queue implements Disposable {
  private static readonly POLL_INTERVAL = 16;

  public readonly functions: Handle["functions"];
  private timer: Option<ReturnType<typeof setTimeout>>;
  private readonly pending = new Map<ffi_RequestId, Pending<unknown>>();

  private disposed = false;

  constructor(handle: Handle) {
    this.functions = handle.functions;
  }

  public [Symbol.dispose]() {
    if (this.disposed) {
      return;
    }

    this.disposed = true;
    this.rejectAll(new HandleClosedError());
  }

  public request<T = unknown>(start: () => ffi_RequestId): Promise<T> {
    if (this.disposed) {
      return Promise.reject(new HandleClosedError());
    }

    return new Promise<T>((resolve, reject) => {
      let id: ffi_RequestId;
      try {
        id = start();
      }
      catch (err) {
        // eslint-disable-next-line @typescript-eslint/prefer-promise-reject-errors
        reject(err);
        return;
      }

      this.pending.set(id, {
        resolve: resolve as Pending<unknown>["resolve"],
        reject,
      });

      this.startPolling();
    });
  }

  private startPolling() {
    if (isNil(this.timer) && !this.disposed) {
      this.timer = setTimeout(() => this.poll(), 0);
    }
  }

  private poll() {
    this.timer = null;
    if (!this.disposed && this.pending.size > 0) {
      try {
        const LIMIT = 200;
        for (let i = 0; i < LIMIT; i++) {
          if (!this.pollOnce()) {
            break;
          }
        }
      }
      catch (err) {
        this.rejectAll(err);
        return;
      }
    }

    if (!this.disposed && this.pending.size > 0) {
      this.timer = setTimeout(() => this.poll(), Queue.POLL_INTERVAL);
    }
  }

  private pollOnce(): boolean {
    const out = allocBuffer();
    const status = this.functions.callofnil_poll(out);

    if (status === ffi_Status.ERR_NOTHING_TO_POLL) {
      return false;
    }

    if (status !== ffi_Status.OK) {
      throw FfiError.fromStatus(status);
    }

    const ptr = readBufferPtr(out);
    if (ptr === 0n) {
      throw new Error("unexpected null pointer");
    }

    let json: string;
    try {
      json = ffi.toString(ptr)!;
    }
    finally {
      this.functions.callofnil_free_str(ptr);
    }

    const entry = JSON.parse(json) as ffi_QueueEntry;
    const response = JSON.parse(entry.json_str) as ffi_Response;

    const pending = this.pending.get(response.id);
    if (!pending) {
      return true;
    }

    this.pending.delete(response.id);

    if (response.kind === "ok") {
      pending.resolve(response.data);
    }
    else {
      pending.reject(
        new FfiError(response.error, {
          status: response.status,
        }),
      );
    }

    return true;
  }

  private rejectAll(error: unknown) {
    if (!isNil(this.timer)) {
      clearTimeout(this.timer);
      this.timer = null;
    }

    for (const { reject } of this.pending.values()) {
      reject(error);
    }

    this.pending.clear();
  }
}

interface Pending<T> {
  resolve: (value: T) => void;
  reject: (reason?: unknown) => void;
}
