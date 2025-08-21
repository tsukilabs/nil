// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export function nextMultipleOf(value: number, multiple: number) {
  return Math.ceil(value / multiple) * multiple;
}
