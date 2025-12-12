// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { clamp } from 'es-toolkit';

export function toI8(value: number) {
  if (!Number.isFinite(value)) return 0;
  return Math.trunc(clamp(value, __CONSTS__.i8Min, __CONSTS__.i16Max));
}

export function toI16(value: number) {
  if (!Number.isFinite(value)) return 0;
  return Math.trunc(clamp(value, __CONSTS__.i16Min, __CONSTS__.i16Max));
}

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

export function toNonZeroU8(value: number) {
  return Math.max(1, toU8(value));
}

export function toNonZeroU16(value: number) {
  return Math.max(1, toU16(value));
}

export function toNonZeroU32(value: number) {
  return Math.max(1, toU32(value));
}
