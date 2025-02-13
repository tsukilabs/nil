use crate::building::prelude::*;
use crate::player::PlayerId;
use bon::Builder;
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
