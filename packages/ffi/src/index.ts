// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as process from "node:process";

export * from "./def";

export function allocBuffer(size?: number): Buffer {
  size ??= process.arch === "x64" ? 8 : 4;
  return Buffer.alloc(size);
}
