// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod behavior;
pub mod city;
pub mod infrastructure;
pub mod military;
pub mod npc;
pub mod player;
pub mod resources;
pub mod round;

pub mod prelude {
  pub use super::behavior::*;
  pub use super::city::*;
  pub use super::infrastructure::*;
  pub use super::military::*;
  pub use super::npc::*;
  pub use super::player::*;
  pub use super::resources::*;
  pub use super::round::*;
}
