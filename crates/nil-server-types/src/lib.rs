use derive_more::{Display, Into};
use nil_core::world::WorldId;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;
use strum::EnumIs;

#[derive(Clone, Copy, Debug, EnumIs, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ServerKind {
  Local { id: WorldId },
  Remote,
}

#[derive(Clone, Debug, Default, Display, Into, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Password(Arc<str>);

impl Password {
  #[inline]
  pub fn new(password: &str) -> Self {
    Self(Arc::from(password))
  }
}

impl Deref for Password {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
