use crate::building::prelude::*;
use crate::player::PlayerId;
use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Village {
  #[builder(start_fn, into)]
  coord: Coord,
  #[builder(into)]
  name: String,
  #[builder(into)]
  owner: Option<PlayerId>,
  #[builder(default)]
  infrastructure: Infrastructure,
}

impl Village {
  pub fn coord(&self) -> Coord {
    self.coord
  }

  pub fn owner(&self) -> Option<PlayerId> {
    self.owner.clone()
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Infrastructure {
  prefecture: Prefecture,
  academy: Academy,
  stable: Stable,
  sawmill: Sawmill,
  quarry: Quarry,
  iron_mine: IronMine,
  farm: Farm,
  warehouse: Warehouse,
  silo: Silo,
  wall: Wall,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Coord {
  x: u8,
  y: u8,
}

impl Coord {
  pub const fn new(x: u8, y: u8) -> Self {
    Self { x, y }
  }

  pub const fn x(&self) -> u8 {
    self.x
  }

  pub const fn y(&self) -> u8 {
    self.y
  }
}

impl From<(u8, u8)> for Coord {
  fn from((x, y): (u8, u8)) -> Self {
    Self::new(x, y)
  }
}
