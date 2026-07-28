/* eslint-disable @typescript-eslint/prefer-promise-reject-errors */

// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Handle } from ".";
import * as ffi from "node:ffi";
import { RequestId } from "./request";
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

type RequestFn = (requestId: ffi_RequestId) => void;

export class Queue implements Disposable {
  private static readonly POLL_INTERVAL = 16;
  private static readonly LIMIT_PER_TICK = 200;

  public readonly functions: Handle["functions"];
  private timer: Option<ReturnType<typeof setTimeout>>;
  private readonly pending = new Map<ffi_RequestId, Pending<unknown>>();

  private disposed = false;
  private readonly requestId = new RequestId();

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

  public request<T = unknown>(start: RequestFn): Promise<T> {
    if (this.disposed) {
      return Promise.reject(new HandleClosedError());
    }

    const requestId = this.requestId.next();
    return new Promise<T>((resolve, reject) => {
      this.pending.set(requestId, {
        resolve: resolve as Pending<unknown>["resolve"],
        reject,
      });

      try {
        start(requestId);
      }
      catch (err) {
        this.pending.delete(requestId);
        reject(err);
        return;
      }

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
        for (let i = 0; i < Queue.LIMIT_PER_TICK; i++) {
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
    const status = this.functions.nil_ffi_poll(out);

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
      this.functions.nil_ffi_free_str(ptr);
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
