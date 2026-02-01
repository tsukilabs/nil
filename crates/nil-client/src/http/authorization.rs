// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::AnyResult;
use derive_more::Deref;
use headers::authorization::Credentials;
use http::HeaderValue;

#[derive(Clone, Debug, Deref)]
pub(crate) struct Authorization(HeaderValue);

impl Authorization {
  pub(crate) fn new<T>(token: T) -> AnyResult<Self>
  where
    T: AsRef<str>,
  {
    let header = headers::Authorization::bearer(token.as_ref())?;
    Ok(Self(header.0.encode()))
  }

  #[inline]
  pub fn as_inner(&self) -> &HeaderValue {
    &self.0
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
