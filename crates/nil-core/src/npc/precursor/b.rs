// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::{ContinentSize, Coord};
use crate::ethic::{EthicPowerAxis, EthicTruthAxis, Ethics};
use crate::npc::precursor::PrecursorId;
use crate::resources::Resources;
use nil_core_macros::Precursor;
use serde::{Deserialize, Serialize};

#[derive(Precursor, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct B {
  origin: Coord,
  resources: Resources,
}

impl B {
  pub const ID: PrecursorId = PrecursorId::B;
  pub const ETHICS: Ethics = Ethics::builder()
    .power(EthicPowerAxis::Pacifist)
    .truth(EthicTruthAxis::Spiritualist)
    .build();

  pub fn new(size: ContinentSize) -> Self {
    Self {
      origin: origin(size),
      resources: Resources::PRECURSOR.clone(),
    }
  }
}

pub fn origin(size: ContinentSize) -> Coord {
  let size = u8::from(size);
  let radius = size.div_ceil(20).next_multiple_of(2);
  Coord::splat(size - 1) - Coord::splat(radius)
}
