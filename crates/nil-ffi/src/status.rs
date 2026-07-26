// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[expect(non_camel_case_types)]
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, repr(enum)))]
#[cfg_attr(feature = "typescript", ts(rename = "ffi_Status"))]
pub enum Status {
  OK = 0,
  ERR_NULL_POINTER = 1,
  ERR_INVALID_UTF8 = 2,
  ERR_SERIALIZATION = 3,
  ERR_PANIC = 4,
  ERR_UNKNOWN = i32::MAX,
}
