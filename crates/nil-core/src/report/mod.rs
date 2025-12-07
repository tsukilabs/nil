// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod battle;

use jiff::Zoned;

pub trait Report {
  fn timestamp(&self) -> &Zoned;
}
