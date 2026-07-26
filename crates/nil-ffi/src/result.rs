// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::string::into_c_string;
use anyhow::Result;
use serde::Serialize;
use std::ffi::c_char;
use std::fmt::Display;
use std::ptr;

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[cfg_attr(feature = "typescript", ts(rename = "ffi_Result"))]
#[cfg_attr(feature = "typescript", ts(concrete(T = serde_json::Value)))]
pub enum FfiResult<T: Serialize> {
  Ok { data: T },
  Err { error: String },
}

impl<T: Serialize> FfiResult<T> {
  pub(crate) fn ok(data: T) -> Self {
    Self::Ok { data }
  }

  pub(crate) fn err<E>(error: E) -> Self
  where
    E: Display,
  {
    Self::Err { error: error.to_string() }
  }
}

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

pub(crate) unsafe fn write<T, F, U>(value: T, out: *mut *mut c_char, f: F) -> Status
where
  F: FnOnce(T) -> U,
  U: Serialize,
{
  if out.is_null() {
    return Status::ERR_NULL_POINTER;
  }

  unsafe { *out = ptr::null_mut() };

  let result = FfiResult::ok(f(value));
  match serialize(result) {
    Ok(json) => {
      unsafe { *out = json };
      Status::OK
    }
    Err(err) => {
      let result = FfiResult::<()>::err(err);
      if let Ok(json) = serialize(result) {
        unsafe { *out = json };
      }

      Status::ERR_SERIALIZATION
    }
  }
}

fn serialize<T: Serialize>(value: T) -> Result<*mut c_char> {
  let json = serde_json::to_string(&value)?;
  Ok(into_c_string(json))
}
