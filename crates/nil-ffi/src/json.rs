// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

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
