// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use std::ffi::{CString, c_char};

pub(crate) fn into_c_string<T>(value: T) -> *mut c_char
where
  T: Into<Vec<u8>>,
{
  CString::new(value)
    .expect("value must not contain nul byte")
    .into_raw()
}
