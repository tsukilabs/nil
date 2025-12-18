// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use derive_more::Deref;
use http::HeaderValue;
use nil_core::player::PlayerId;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use std::fmt;

#[derive(Clone, Debug, Deref)]
pub(crate) struct Authorization(HeaderValue);

impl Authorization {
  pub fn into_inner(self) -> HeaderValue {
    self.0
  }
}

impl From<Authorization> for HeaderValue {
  fn from(authorization: Authorization) -> Self {
    authorization.into_inner()
  }
}

impl TryFrom<&PlayerId> for Authorization {
  type Error = Error;

  fn try_from(id: &PlayerId) -> Result<Self> {
    let player = utf8_percent_encode(id, NON_ALPHANUMERIC).to_string();
    let Ok(authorization) = HeaderValue::try_from(&player) else {
      return Err(Error::InvalidPlayerId(id.clone()));
    };

    Ok(Self(authorization))
  }
}

impl fmt::Display for Authorization {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", String::from_utf8_lossy(self.0.as_bytes()))
  }
}
