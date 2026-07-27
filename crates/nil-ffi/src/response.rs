// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::request::RequestId;
use crate::status::Status;
use serde::Serialize;
use std::fmt::Display;

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[cfg_attr(feature = "typescript", ts(rename = "ffi_Response"))]
#[cfg_attr(feature = "typescript", ts(concrete(T = serde_json::Value)))]
pub struct Response<T>
where
  T: Serialize,
{
  pub id: RequestId,
  #[serde(flatten)]
  pub result: Result<T>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[cfg_attr(feature = "typescript", ts(rename = "ffi_Result"))]
#[cfg_attr(feature = "typescript", ts(concrete(T = serde_json::Value)))]
pub enum Result<T: Serialize> {
  Ok { data: T },
  Err { status: Status, error: String },
}

impl<T: Serialize> Result<T> {
  pub(crate) fn ok(data: T) -> Self {
    Self::Ok { data }
  }

  pub(crate) fn err<E>(error: E) -> Self
  where
    E: Display,
  {
    Self::status(Status::ERR_UNKNOWN, error)
  }

  pub(crate) fn status<E>(status: Status, error: E) -> Self
  where
    E: Display,
  {
    Self::Err { status, error: error.to_string() }
  }
}

impl<T, E> From<std::result::Result<T, E>> for Result<T>
where
  T: Serialize,
  E: Display,
{
  fn from(value: std::result::Result<T, E>) -> Self {
    match value {
      Ok(data) => Self::ok(data),
      Err(error) => Self::err(error),
    }
  }
}
