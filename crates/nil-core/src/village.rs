use crate::building::prelude::*;

#[derive(Debug)]
pub struct Village {
  pub name: String,
  pub infrastructure: Infrastructure,
}

#[derive(Debug)]
pub struct Infrastructure {
  pub prefecture: Prefecture,
  pub stable: Stable,
  pub sawmill: Sawmill,
  pub quarry: Quarry,
  pub iron_mine: IronMine,
  pub farm: Farm,
  pub warehouse: Warehouse,
  pub wall: Wall,
}
