// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::{ContinentSize, Coord};
use crate::ethic::{EthicPowerAxis, EthicTruthAxis, Ethics};
use crate::npc::precursor::PrecursorId;
use crate::resource::Resources;
use nil_core_macros::Precursor;
use serde::{Deserialize, Serialize};

#[derive(Precursor, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct A {
  origin: Coord,
  resources: Resources,
}

impl A {
  pub const ID: PrecursorId = PrecursorId::A;
  pub const ETHICS: Ethics = Ethics::builder()
    .power(EthicPowerAxis::Militarist)
    .truth(EthicTruthAxis::Materialist)
    .build();

  pub fn new(size: ContinentSize) -> Self {
    Self {
      origin: origin(size),
      resources: Resources::PRECURSOR.clone(),
    }
  }
}

fn origin(size: ContinentSize) -> Coord {
  let size = u8::from(size);
  let radius = size.div_ceil(20).next_multiple_of(2);
  Coord::splat(0) + Coord::splat(radius)
}
