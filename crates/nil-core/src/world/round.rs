use super::World;
use crate::round::RoundState;

impl World {
  pub fn round_state(&self) -> RoundState {
    self.round.state()
  }
}
