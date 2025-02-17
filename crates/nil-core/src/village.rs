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
  owner: VillageOwner,
  #[builder(default)]
  infrastructure: Infrastructure,
}

impl Village {
  pub fn coord(&self) -> Coord {
    self.coord
  }

  pub fn owner(&self) -> VillageOwner {
    self.owner.clone()
  }

  pub fn is_owned_by(&self, player: &PlayerId) -> bool {
    self
      .owner
      .player()
      .is_some_and(|id| id == player)
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum VillageOwner {
  #[default]
  None,
  Player {
    id: PlayerId,
  },
}

impl VillageOwner {
  pub fn player(&self) -> Option<&PlayerId> {
    if let Self::Player { id } = self {
      Some(id)
    } else {
      None
    }
  }
}

impl From<PlayerId> for VillageOwner {
  fn from(id: PlayerId) -> Self {
    Self::Player { id }
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
