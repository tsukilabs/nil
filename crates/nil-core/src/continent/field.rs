// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::village::{PublicVillage, Village};
use serde::{Deserialize, Serialize};
use strum::EnumIs;

#[derive(Clone, Debug, Default, Deserialize, Serialize, EnumIs)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Field {
  #[default]
  Empty,
  Village {
    village: Village,
  },
}

impl Field {
  pub(super) fn village(&self) -> Option<&Village> {
    if let Self::Village { village } = self {
      Some(village)
    } else {
      None
    }
  }

  pub(super) fn village_mut(&mut self) -> Option<&mut Village> {
    if let Self::Village { village } = self {
      Some(village)
    } else {
      None
    }
  }
}

impl From<Village> for Field {
  fn from(village: Village) -> Self {
    Self::Village { village }
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, EnumIs)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum PublicField {
  #[default]
  Empty,
  Village {
    village: PublicVillage,
  },
}

impl From<&Field> for PublicField {
  fn from(field: &Field) -> Self {
    match field {
      Field::Empty => Self::Empty,
      Field::Village { village } => {
        Self::Village {
          village: PublicVillage::from(village),
        }
      }
    }
  }
}
