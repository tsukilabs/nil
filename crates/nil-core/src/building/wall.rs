use super::BuildingLevel;
use nil_core_macros::Building;
use serde::{Deserialize, Serialize};

#[derive(Building, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Wall {
  level: BuildingLevel,
}

impl Wall {
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(20);
}

impl Default for Wall {
  fn default() -> Self {
    Self { level: BuildingLevel::new(0) }
  }
}
