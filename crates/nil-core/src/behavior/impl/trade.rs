// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::ruler::Ruler;
use bon::Builder;

#[derive(Builder, Debug)]
pub struct TradeBehavior {
  ruler: Ruler,
}

impl TradeBehavior {}
