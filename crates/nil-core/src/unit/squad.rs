use derive_more::Deref;

#[derive(Clone, Copy, Debug, Deref)]
pub struct SquadAttack(f64);

impl SquadAttack {
  pub fn new(value: f64) -> Self {
    Self(value.max(0.0))
  }
}

impl From<f64> for SquadAttack {
  fn from(value: f64) -> Self {
    Self::new(value)
  }
}

pub struct SquadDefense {
  pub infantry: f64,
  pub cavalry: f64,
  pub ranged: f64,
}
