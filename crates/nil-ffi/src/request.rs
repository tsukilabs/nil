// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use serde::Serialize;
use std::ffi::c_uint;

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[cfg_attr(feature = "typescript", ts(rename = "ffi_RequestId"))]
pub struct RequestId(c_uint);
