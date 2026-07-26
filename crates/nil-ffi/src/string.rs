// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use serde::Serialize;
use std::ffi::{CString, c_char};

pub(crate) fn into_c_string<T>(value: T) -> *mut c_char
where
  T: Into<Vec<u8>>,
{
  CString::new(value)
    .expect("value must not contain nul byte")
    .into_raw()
}

pub(crate) fn serialize<T: Serialize>(value: T) -> Result<CString> {
  let json = serde_json::to_string(&value)?;
  Ok(CString::new(json)?)
}
