// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use serde::Serialize;
use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(feature = "typescript")]
use ts_rs::TS;

static REQUEST_ID: AtomicU32 = AtomicU32::new(1);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[cfg_attr(feature = "typescript", ts(rename = "ffi_RequestId"))]
pub struct RequestId(u32);

pub(crate) fn next_request_id() -> RequestId {
  RequestId(REQUEST_ID.fetch_add(1, Ordering::Relaxed))
}
