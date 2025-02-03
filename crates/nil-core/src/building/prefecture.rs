use super::BuildingLevel;
use nil_macros::Building;
use serde::{Deserialize, Serialize};

#[derive(Building, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Prefecture {
  level: BuildingLevel,
}

impl Prefecture {
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(30);
}

impl Default for Prefecture {
  fn default() -> Self {
    Self { level: BuildingLevel::new(1) }
  }
}
