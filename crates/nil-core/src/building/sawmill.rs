use super::BuildingLevel;
use nil_macros::Building;

#[derive(Building, Debug)]
pub struct Sawmill {
  level: BuildingLevel,
}

impl Sawmill {
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(30);
}
