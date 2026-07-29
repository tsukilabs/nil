// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::{RequestId, Status, queue};
use anyhow::Result;
use serde::de::DeserializeOwned;
use std::ffi::{CStr, c_char};

pub(crate) use serde_json::{from_str as deserialize, to_string as serialize};

pub(crate) unsafe fn deserialize_ptr<T>(ptr: *const c_char) -> Result<T>
where
  T: DeserializeOwned,
{
  let cstr = unsafe { CStr::from_ptr(ptr) };
  Ok(deserialize(cstr.to_str()?)?)
}

pub(crate) unsafe fn with_ptr<T, F>(request_id: RequestId, ptr: *const c_char, f: F)
where
  T: DeserializeOwned,
  F: FnOnce(T),
{
  if ptr.is_null() {
    queue::push_err(request_id, Status::ERR_NULL_POINTER);
  } else {
    match unsafe { deserialize_ptr::<T>(ptr) } {
      Ok(value) => f(value),
      Err(err) => queue::push_err(request_id, err),
    }
  }
}
