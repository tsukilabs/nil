// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::From;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::ops::Deref;

#[derive(Clone, Debug, From, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[from(String, &str, Box<str>, Cow<'_, str>)]
pub struct Token(Box<str>);

impl Token {
  pub fn new(token: impl AsRef<str>) -> Self {
    Self(Box::from(token.as_ref()))
  }
}

impl AsRef<str> for Token {
  fn as_ref(&self) -> &str {
    self.0.as_str()
  }
}

impl AsRef<[u8]> for Token {
  fn as_ref(&self) -> &[u8] {
    self.0.as_bytes()
  }
}

impl Deref for Token {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    self.0.as_str()
  }
}
