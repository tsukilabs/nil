use super::BuildingLevel;
use nil_macros::Building;

#[derive(Building, Debug)]
pub struct Academy {
  level: BuildingLevel,
}

impl Academy {
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(25);
}
