use super::BuildingLevel;
use nil_macros::Building;
use serde::{Deserialize, Serialize};

#[derive(Building, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Silo {
  level: BuildingLevel,
}

impl Silo {
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(30);
}

impl Default for Silo {
  fn default() -> Self {
    Self { level: BuildingLevel::new(1) }
  }
}
