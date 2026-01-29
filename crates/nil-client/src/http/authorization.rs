// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::AnyResult;
use derive_more::Deref;
use http::HeaderValue;
use std::fmt;

#[derive(Clone, Debug, Deref)]
pub(crate) struct Authorization(HeaderValue);

impl Authorization {
  pub(crate) fn new<T>(token: T) -> AnyResult<Self>
  where
    T: AsRef<str>,
  {
    Ok(Self(HeaderValue::from_str(token.as_ref())?))
  }

  #[inline]
  pub fn into_inner(self) -> HeaderValue {
    self.0
  }
}

impl From<Authorization> for HeaderValue {
  fn from(authorization: Authorization) -> Self {
    authorization.into_inner()
  }
}

impl fmt::Display for Authorization {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", String::from_utf8_lossy(self.0.as_bytes()))
  }
}
