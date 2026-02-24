// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

/**
 * Checks if the string includes any char that could make its use as a path segment unsafe.
 *
 * @see https://docs.rs/url/latest/url/struct.Url.html#method.parse
 * @see https://docs.rs/url/latest/url/struct.PathSegmentsMut.html#method.extend
 */
export function isSafePathSegment(value: string) {
  const chars = ['@', '#', '$', '%', '&', '=', '/', '\\'];
  return chars.every((c) => !value.includes(c));
}
