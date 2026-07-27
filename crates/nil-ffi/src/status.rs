// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use strum::Display;

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[expect(non_camel_case_types)]
#[repr(i32)]
#[derive(Clone, Copy, Debug, Display, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, repr(enum)))]
#[cfg_attr(feature = "typescript", ts(rename = "ffi_Status"))]
pub enum Status {
  OK = 0,
  ERR_NULL_POINTER = 1,
  ERR_NOTHING_TO_POLL = 2,
  ERR_INVALID_UTF8 = 3,
  ERR_SERIALIZATION = 4,
  ERR_UNKNOWN = i32::MAX,
}
