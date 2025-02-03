use super::BuildingLevel;
use nil_macros::Building;
use serde::{Deserialize, Serialize};

#[derive(Building, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Warehouse {
  level: BuildingLevel,
}

impl Warehouse {
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(30);
}

impl Default for Warehouse {
  fn default() -> Self {
    Self { level: BuildingLevel::new(1) }
  }
}
