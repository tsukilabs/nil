// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::string::into_c_string;
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::ffi::c_char;
use std::ptr;

#[derive(Debug, Serialize)]
pub struct FfiResult {
  pub data: JsonValue,
}

impl FfiResult {
  pub(crate) fn new<T>(data: T) -> Result<Self, Status>
  where
    T: Serialize,
  {
    match serde_json::to_value(data) {
      Ok(data) => Ok(Self { data }),
      Err(_) => Err(Status::ERR_SERIALIZATION),
    }
  }
}

#[expect(non_camel_case_types)]
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
  T: Serialize,
  F: FnOnce(T) -> U,
  U: Serialize,
{
  if out.is_null() {
    Status::ERR_NULL_POINTER
  } else {
    unsafe { *out = ptr::null_mut() };

    match FfiResult::new(f(value)) {
      Ok(result) => {
        match serde_json::to_string(&result) {
          Ok(json) => {
            unsafe { *out = into_c_string(json) };
            Status::OK
          }
          Err(_) => Status::ERR_SERIALIZATION,
        }
      }
      Err(status) => status,
    }
  }
}
