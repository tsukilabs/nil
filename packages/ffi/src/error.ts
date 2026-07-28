// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from "@tb-dev/utils";
import { type ffi_Response, ffi_Status } from "@tsukilabs/nil-bindings";

export interface FfiErrorOptions extends ErrorOptions {
  readonly status?: Option<ffi_Status>;
}

export class NilError extends Error {
  public override readonly name = "NilError";
  public readonly status: ffi_Status;

  constructor(message: Option<string>, options?: FfiErrorOptions) {
    const status = options?.status ?? ffi_Status.ERR_UNKNOWN;
    if (!message && status !== ffi_Status.OK) {
      for (const [key, value] of Object.entries(ffi_Status)) {
        if (value === status) {
          message = key;
          break;
        }
      }
    }

    message ??= "ERR_UNKNOWN" satisfies keyof typeof ffi_Status;
    super(message, options);

    this.status = status;
  }

  public static fromStatus(status: ffi_Status) {
    return new NilError(null, { status });
  }
}

export class HandleClosedError extends Error {
  public override readonly name = "HandleClosedError";

  constructor(options?: ErrorOptions) {
    super("handle is closed", options);
  }
}

export class UnknownResponseError extends Error {
  public override readonly name = "UnknownResponseError";
  public readonly response: ffi_Response;

  constructor(response: ffi_Response, options?: ErrorOptions) {
    super(`got unknown response with id ${response.id}`, options);
    this.response = response;
  }
}
