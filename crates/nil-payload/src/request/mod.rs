// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod auth;
pub mod battle;
pub mod chat;
pub mod cheat;
pub mod city;
pub mod continent;
pub mod infrastructure;
pub mod military;
pub mod npc;
pub mod player;
pub mod ranking;
pub mod report;
pub mod round;
pub mod user;
pub mod world;

pub mod prelude {
  pub use super::auth::*;
  pub use super::battle::*;
  pub use super::chat::*;
  pub use super::cheat::prelude::*;
  pub use super::city::*;
  pub use super::continent::*;
  pub use super::infrastructure::prelude::*;
  pub use super::military::*;
  pub use super::npc::prelude::*;
  pub use super::player::*;
  pub use super::ranking::*;
  pub use super::report::*;
  pub use super::round::*;
  pub use super::user::*;
  pub use super::world::*;
}
