// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from "@tb-dev/utils";
import type { ffi_Status } from "@tsukilabs/nil-bindings";

export interface FfiErrorOptions extends ErrorOptions {
  readonly status: ffi_Status;
}

export class FfiError extends Error {
  public override readonly name = "FfiError";
  public readonly status: ffi_Status;

  constructor(message: Option<string>, options: FfiErrorOptions) {
    message ??= "ERR_UNKNOWN" satisfies keyof typeof ffi_Status;
    super(message, options);

    this.status = options.status;
  }
}
