// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { clamp } from 'es-toolkit';

export function toU8(value: number) {
  if (!Number.isFinite(value)) return 0;
  return Math.trunc(clamp(value, 0, __CONSTS__.u8Max));
}

export function toU16(value: number) {
  if (!Number.isFinite(value)) return 0;
  return Math.trunc(clamp(value, 0, __CONSTS__.u16Max));
}

export function toU32(value: number) {
  if (!Number.isFinite(value)) return 0;
  return Math.trunc(clamp(value, 0, __CONSTS__.u32Max));
}
