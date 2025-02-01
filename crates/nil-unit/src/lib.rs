mod index;
pub mod prelude;

use nil_core::{Error, Result, UnitBox, UnitId};

pub fn new<Id>(id: Id, amount: u32) -> Result<UnitBox>
where
  Id: Into<UnitId>,
{
  use prelude::*;

  let id: UnitId = id.into();
  let skill = match id.get() {
    1 => Pikeman::new_boxed(amount),
    2 => Swordsman::new_boxed(amount),
    3 => Axeman::new_boxed(amount),
    4 => Archer::new_boxed(amount),
    5 => LightCavalry::new_boxed(amount),
    6 => HeavyCavalry::new_boxed(amount),
    _ => return Err(Error::UnitNotFound(id)),
  };

  Ok(skill)
}
