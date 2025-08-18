// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::city::{City, PublicCity};
use serde::{Deserialize, Serialize};
use strum::EnumIs;

#[derive(Clone, Debug, Default, Deserialize, Serialize, EnumIs)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Field {
  #[default]
  Empty,
  City {
    city: City,
  },
}

impl Field {
  #[inline]
  pub fn city(&self) -> Option<&City> {
    if let Self::City { city } = self { Some(city) } else { None }
  }

  pub(super) fn city_mut(&mut self) -> Option<&mut City> {
    if let Self::City { city } = self { Some(city) } else { None }
  }
}

impl From<City> for Field {
  fn from(city: City) -> Self {
    Self::City { city }
  }
}

/// Public data about a field, to which any player can have access.
#[derive(Clone, Debug, Default, Deserialize, Serialize, EnumIs)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum PublicField {
  #[default]
  Empty,
  City {
    city: PublicCity,
  },
}

impl From<&Field> for PublicField {
  fn from(field: &Field) -> Self {
    match field {
      Field::Empty => Self::Empty,
      Field::City { city } => Self::City { city: PublicCity::from(city) },
    }
  }
}
