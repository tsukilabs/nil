use super::BuildingLevel;
use nil_core_macros::Building;
use serde::{Deserialize, Serialize};

#[derive(Building, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stable {
  level: BuildingLevel,
}

impl Stable {
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(20);
}

impl Default for Stable {
  fn default() -> Self {
    Self { level: BuildingLevel::new(0) }
  }
}
