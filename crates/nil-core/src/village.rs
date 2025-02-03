use crate::building::prelude::*;
use crate::player::PlayerId;
use crate::world::Coord;
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
  pub wall: Wall,
}
