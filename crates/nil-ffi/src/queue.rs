// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::request::RequestId;
use crate::response::{FfiResponse, FfiResult};
use serde::Serialize;
use serde_json::to_string as serialize;
use std::collections::VecDeque;
use std::fmt::Display;
use std::sync::LazyLock;
use std::sync::nonpoison::Mutex;

#[cfg(feature = "typescript")]
use ts_rs::TS;

static QUEUE: LazyLock<Mutex<VecDeque<QueueEntry>>> = LazyLock::new(Mutex::default);

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[cfg_attr(feature = "typescript", ts(rename = "ffi_QueueEntry"))]
pub struct QueueEntry {
  kind: QueueEntryKind,
  json_str: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[cfg_attr(feature = "typescript", ts(rename = "ffi_QueueEntryKind"))]
pub enum QueueEntryKind {
  Response,
}

pub(crate) fn poll() -> Option<QueueEntry> {
  QUEUE.lock().pop_front()
}

pub(crate) fn push_result<T>(id: RequestId, result: FfiResult<T>)
where
  T: Serialize,
{
  let response = FfiResponse { id, result };
  let kind = QueueEntryKind::Response;
  let entry = match serialize(&response) {
    Ok(json_str) => QueueEntry { kind, json_str },
    Err(err) => {
      let result = FfiResult::<()>::err(err);
      let response = FfiResponse { id, result };
      let json_str = serialize(&response).expect("`FfiResult<()>` must always serialize");
      QueueEntry { kind, json_str }
    }
  };

  QUEUE.lock().push_back(entry);
}

pub(crate) fn push_ok<T>(id: RequestId, data: T)
where
  T: Serialize,
{
  push_result(id, FfiResult::ok(data));
}

pub(crate) fn push_err<E>(id: RequestId, error: E)
where
  E: Display,
{
  push_result(id, FfiResult::<()>::err(error));
}
