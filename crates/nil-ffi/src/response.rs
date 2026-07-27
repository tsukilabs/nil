// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::request::RequestId;
use anyhow::Result;
use serde::Serialize;
use std::fmt::Display;

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
