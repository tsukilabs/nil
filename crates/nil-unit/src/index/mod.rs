pub(crate) mod archer;
pub(crate) mod axeman;
pub(crate) mod heavy_cavalry;
pub(crate) mod light_cavalry;
pub(crate) mod pikeman;
pub(crate) mod swordsman;

mod prelude {
  pub use nil_macros::Unit;

  pub use nil_core::{Haul, Power, Speed, UnitId, UnitKind, UnitStats};
}
