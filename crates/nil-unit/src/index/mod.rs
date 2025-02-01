pub(crate) mod pikeman;
pub(crate) mod swordsman;
pub (crate) mod axeman;
pub (crate) mod archer;
pub (crate) mod light_cavalry;
pub (crate) mod heavy_cavalry;


mod prelude {
  pub use nil_macros::Unit;

  pub use nil_core::{UnitKind, Haul, Power, Speed, UnitId, UnitStats};
}
