// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

const ZONE_REGEX = /\[.+?\]/;

/**
 * Example:
 *
 * 2025-06-02T22:08:36.6360753-03:00[America/Sao_Paulo]
 */
export function fromZoned(zoned: string) {
  return new Date(zoned.replace(ZONE_REGEX, ''));
}
