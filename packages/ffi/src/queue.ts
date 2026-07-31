/* eslint-disable @typescript-eslint/prefer-promise-reject-errors */

//! Copyright (C) Call of Nil contributors
//! SPDX-License-Identifier: AGPL-3.0-only

import type { Handle } from ".";
import * as ffi from "node:ffi";
import { RequestId } from "./request";
import { EventEmitter } from "node:events";
import { isNil } from "es-toolkit/predicate";
import type { nil, Option } from "@tb-dev/utils";
import { allocBuffer, readBufferPtr } from "./ptr";
import { HandleClosedError, NilError, UnknownResponseError } from "./error";
import {
  type ffi_QueueEntry,
  type ffi_RequestId,
  type ffi_Response,
  ffi_Status,
} from "@tsukilabs/nil-bindings";

class QueueEmitter extends EventEmitter {
  public override emit(event: "error", error: unknown): boolean;
  public override emit(event: "response", response: ffi_Response): boolean;
  public override emit(event: QueueEvent, ...args: any[]): boolean {
    return super.emit(event, ...args);
  }

  public override on(event: "error", listener: (error: unknown) => void): this;
  public override on(event: "response", listener: (response: ffi_Response) => void): this;
  public override on(event: QueueEvent, listener: (...args: any[]) => void): this {
    return super.on(event, listener);
  }
}

export class Queue implements Disposable {
  private static readonly POLL_INTERVAL = 16;
  private static readonly LIMIT_PER_TICK = 200;

  private disposed = false;
  private readonly functions: Handle["functions"];

  private readonly emitter = new QueueEmitter();
  private readonly requestId = new RequestId();
  private readonly pendingRequests = new Map<ffi_RequestId, Pending<unknown>>();
  private timer: Option<ReturnType<typeof setTimeout>>;

  private readonly errorHandler: ErrorHandler;

  constructor(handle: Handle, options?: QueueOptions) {
    this.functions = handle.functions;
    this.errorHandler = options?.onError ?? console.error;

    this.emitter.on("error", this.errorHandler);
    this.emitter.on("response", this.onResponse.bind(this));

    this.timer = setTimeout(() => this.drain(), 0);
  }

  public [Symbol.dispose]() {
    if (this.disposed) {
      return;
    }

    this.disposed = true;
    this.rejectAll(new HandleClosedError());
    this.emitter.removeAllListeners();
  }

  public request<T = nil>(start: RequestFn): Promise<T> {
    if (this.disposed) {
      return Promise.reject(new HandleClosedError());
    }

    const requestId = this.requestId.next();
    return new Promise<T>((resolve, reject) => {
      this.pendingRequests.set(requestId, {
        resolve: resolve as Pending<unknown>["resolve"],
        reject,
      });

      try {
        start(requestId);
      }
      catch (err) {
        this.pendingRequests.delete(requestId);
        reject(err);
      }
    });
  }

  private drain() {
    if (this.disposed) {
      return;
    }

    this.clearTimeout();

    try {
      for (let i = 0; i < Queue.LIMIT_PER_TICK; i++) {
        if (!this.poll()) break;
      }
    }
    catch (err) {
      this.emitter.emit("error", err);
    }

    this.timer = setTimeout(() => this.drain(), Queue.POLL_INTERVAL);
  }

  private poll(): boolean {
    const out = allocBuffer();
    const status = this.functions.nil_ffi_poll(out);

    if (status === ffi_Status.ERR_NOTHING_TO_POLL) {
      return false;
    }

    if (status !== ffi_Status.OK) {
      throw NilError.fromStatus(status);
    }

    const ptr = readBufferPtr(out);
    if (ptr === 0n) {
      throw new Error("failed to poll: unexpected null pointer");
    }

    try {
      const json = ffi.toString(ptr) ?? "";
      const entry = JSON.parse(json) as ffi_QueueEntry;
      this.dispatch(entry);
    }
    finally {
      this.functions.nil_ffi_free_str(ptr);
    }

    return true;
  }

  private dispatch(entry: ffi_QueueEntry) {
    switch (entry.kind) {
      // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition
      case "response": {
        const response = JSON.parse(entry.jsonStr) as ffi_Response;
        this.emitter.emit("response", response);
        break;
      }
    }
  }

  private rejectAll(error: unknown) {
    this.clearTimeout();
    this.pendingRequests.values()
      .forEach(({ reject }) => reject(error));

    this.pendingRequests.clear();
  }

  private clearTimeout() {
    if (!isNil(this.timer)) {
      clearTimeout(this.timer);
      this.timer = null;
    }
  }

  private onResponse(response: ffi_Response) {
    const pending = this.pendingRequests.get(response.id);
    if (pending) {
      this.pendingRequests.delete(response.id);

      if (response.kind === "ok") {
        pending.resolve(response.data);
      }
      else {
        const { status, error } = response;
        pending.reject(new NilError(error, { status }));
      }
    }
    else {
      this.emitter.emit("error", new UnknownResponseError(response));
    }
  }
}

type QueueEvent = "error" | "response";
type RequestFn = (requestId: ffi_RequestId) => void;

export type ErrorHandler = (error: unknown) => void;

export interface QueueOptions {
  onError?: Option<ErrorHandler>;
}

interface Pending<T> {
  resolve: (value: T) => void;
  reject: (reason?: unknown) => void;
}
