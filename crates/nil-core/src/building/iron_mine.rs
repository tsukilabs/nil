use super::BuildingLevel;
use nil_core_macros::Building;
use serde::{Deserialize, Serialize};

#[derive(Building, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IronMine {
  level: BuildingLevel,
}

impl IronMine {
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(30);
}

impl Default for IronMine {
  fn default() -> Self {
    Self { level: BuildingLevel::new(1) }
  }
}
