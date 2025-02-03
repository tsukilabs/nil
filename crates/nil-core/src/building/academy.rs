use super::BuildingLevel;
use nil_macros::Building;
use serde::{Deserialize, Serialize};

#[derive(Building, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Academy {
  level: BuildingLevel,
}

impl Academy {
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(25);
}

impl Default for Academy {
  fn default() -> Self {
    Self { level: BuildingLevel::new(0) }
  }
}
