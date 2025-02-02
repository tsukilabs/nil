use super::BuildingLevel;
use nil_macros::Building;

#[derive(Building, Debug)]
pub struct IronMine {
  level: BuildingLevel,
}

impl IronMine {
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(30);
}
