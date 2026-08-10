// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::coord::Coord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Capital {
  pub(crate) coord: Option<Coord>,
}

impl Capital {
  #[inline]
  pub fn new(coord: Coord) -> Self {
    Self { coord: Some(coord) }
  }

  #[inline]
  pub fn coord(&self) -> Option<Coord> {
    self.coord
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct PublicCapital {
  coord: Option<Coord>,
}

impl From<&Capital> for PublicCapital {
  fn from(capital: &Capital) -> Self {
    Self { coord: capital.coord }
  }
}
