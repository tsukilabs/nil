#![feature(str_as_str)]

use nil_core::world::WorldId;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use strum::EnumIs;

#[derive(Clone, Copy, Debug, EnumIs, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ServerKind {
  Local { id: WorldId },
  Remote,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
