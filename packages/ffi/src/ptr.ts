// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as process from "node:process";

export function allocBuffer(size?: number) {
  return Buffer.alloc(size ?? pointerSize());
}

export function readBufferPtr(buffer: Buffer) {
  if (pointerSize() === 8) {
    return buffer.readBigUInt64LE();
  }
  else {
    return BigInt(buffer.readUInt32LE());
  }
}

function pointerSize() {
  if (process.arch === "x64" || process.arch === "arm64") {
    return 8;
  }
  else {
    return 4;
  }
}
