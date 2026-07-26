// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::request::RequestId;
use crate::status::Status;
use crate::string::serialize;
use anyhow::Result;
use serde::Serialize;
use std::ffi::c_char;
use std::fmt::Display;
use std::ptr;

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[cfg_attr(feature = "typescript", ts(rename = "ffi_Response"))]
#[cfg_attr(feature = "typescript", ts(concrete(T = serde_json::Value)))]
pub struct FfiResponse<T>
where
  T: Serialize,
{
  pub id: RequestId,
  #[serde(flatten)]
  pub result: FfiResult<T>,
}

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

impl<T, E> From<Result<T, E>> for FfiResult<T>
where
  T: Serialize,
  E: Display,
{
  fn from(value: Result<T, E>) -> Self {
    match value {
      Ok(data) => Self::ok(data),
      Err(error) => Self::err(error),
    }
  }
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
      unsafe { *out = json.into_raw() };
      Status::OK
    }
    Err(err) => {
      let result = FfiResult::<()>::err(err);
      if let Ok(json) = serialize(result) {
        unsafe { *out = json.into_raw() };
      }

      Status::ERR_SERIALIZATION
    }
  }
}
