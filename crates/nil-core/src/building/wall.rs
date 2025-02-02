use super::BuildingLevel;
use nil_macros::Building;

#[derive(Building, Debug)]
pub struct Wall {
  level: BuildingLevel,
}

impl Wall {
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(20);
}
