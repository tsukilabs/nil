// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::request::RequestId;
use crate::response::{Response, Result};
use crate::status::Status;
use nil_core::event::Event;
use serde::Serialize;
use serde_json::to_string as serialize;
use std::collections::VecDeque;
use std::fmt::Display;
use std::sync::LazyLock;
use std::sync::nonpoison::Mutex;

static QUEUE: LazyLock<Mutex<VecDeque<QueueEntry>>> = LazyLock::new(Mutex::default);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[cfg_attr(feature = "typescript", ts(rename = "ffi_QueueEntry"))]
pub struct QueueEntry {
  kind: QueueEntryKind,
  json_str: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[cfg_attr(feature = "typescript", ts(rename = "ffi_QueueEntryKind"))]
pub enum QueueEntryKind {
  Event,
  Response,
}

pub(crate) fn poll() -> Option<QueueEntry> {
  QUEUE.lock().pop_front()
}

pub(crate) fn clear() {
  let mut queue = QUEUE.lock();
  queue.clear();
  queue.shrink_to_fit();
}

pub(crate) fn push_event(event: &Event) {
  let entry = QueueEntry {
    kind: QueueEntryKind::Event,
    json_str: serialize(event).expect("`Event` must always serialize"),
  };

  QUEUE.lock().push_back(entry);
}

pub(crate) fn push_result<T>(id: RequestId, result: Result<T>)
where
  T: Serialize,
{
  let response = Response { id, result };
  let kind = QueueEntryKind::Response;
  let entry = match serialize(&response) {
    Ok(json_str) => QueueEntry { kind, json_str },
    Err(err) => {
      let result = Result::<()>::err_with_status(err, Status::ERR_SERIALIZATION);
      let response = Response { id, result };
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
  push_result(id, Result::ok(data));
}

pub(crate) fn push_err<E>(id: RequestId, error: E)
where
  E: Display,
{
  push_result(id, Result::<()>::err(error));
}
