mod academy;
mod farm;
mod iron_mine;
mod prefecture;
mod quarry;
mod sawmill;
mod stable;
mod wall;
mod warehouse;

pub mod prelude {
  pub use super::academy::Academy;
  pub use super::farm::Farm;
  pub use super::iron_mine::IronMine;
  pub use super::prefecture::Prefecture;
  pub use super::quarry::Quarry;
  pub use super::sawmill::Sawmill;
  pub use super::stable::Stable;
  pub use super::wall::Wall;
  pub use super::warehouse::Warehouse;
}

use derive_more::Deref;
use std::num::NonZeroU8;

pub trait Building {
  fn level(&self) -> BuildingLevel;
  fn max_level(&self) -> BuildingLevel;
}

#[derive(Clone, Copy, Debug, Deref)]
pub struct BuildingId(NonZeroU8);

impl BuildingId {
  pub const fn new(id: u8) -> Self {
    Self(NonZeroU8::new(id).unwrap())
  }
}

#[derive(Clone, Copy, Debug, Default, Deref)]
pub struct BuildingLevel(u8);

impl BuildingLevel {
  pub const fn new(level: u8) -> Self {
    Self(level)
  }
}
