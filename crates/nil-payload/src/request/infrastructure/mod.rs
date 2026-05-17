// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod academy;
pub mod building;
pub mod prefecture;
pub mod stable;
pub mod workshop;

pub mod prelude {
  pub use super::academy::*;
  pub use super::building::*;
  pub use super::prefecture::*;
  pub use super::stable::*;
  pub use super::workshop::*;
}
