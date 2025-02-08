use crate::building::prelude::*;
use crate::player::PlayerId;
use bon::Builder;
use glam::U8Vec2;
use serde::{Deserialize, Serialize};

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Village {
  #[builder(start_fn, into)]
  pub coord: Coord,
  #[builder(into)]
  pub name: String,
  #[builder(into)]
  pub owner: Option<PlayerId>,
  #[builder(default)]
  pub infrastructure: Infrastructure,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Infrastructure {
  pub prefecture: Prefecture,
  pub academy: Academy,
  pub stable: Stable,
  pub sawmill: Sawmill,
  pub quarry: Quarry,
  pub iron_mine: IronMine,
  pub farm: Farm,
  pub warehouse: Warehouse,
  pub silo: Silo,
  pub wall: Wall,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Coord(U8Vec2);

impl Coord {
  pub const fn new(x: u8, y: u8) -> Self {
    Self(U8Vec2::new(x, y))
  }

  pub const fn x(&self) -> u8 {
    self.0.x
  }

  pub const fn y(&self) -> u8 {
    self.0.y
  }
}

impl From<(u8, u8)> for Coord {
  fn from((x, y): (u8, u8)) -> Self {
    Self::new(x, y)
  }
}
