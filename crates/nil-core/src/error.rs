use crate::event::EventError;
use crate::player::PlayerId;
use crate::unit::UnitId;
use crate::village::Coord;
use serde::Serialize;
use serde::ser::Serializer;
use strum::EnumIs;

pub use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

#[non_exhaustive]
#[derive(Debug, EnumIs, thiserror::Error)]
pub enum Error {
  #[error("coord out of bounds: {0:?}")]
  CoordOutOfBounds(Coord),
  #[error("index out of bounds: {0}")]
  IndexOutOfBounds(usize),
  #[error("not a village: {0:?}")]
  NotAVillage(Coord),
  #[error("no player to schedule")]
  NoPlayerToSchedule,
  #[error("player already exists")]
  PlayerAlreadyExists,
  #[error("player not found: {0}")]
  PlayerNotFound(PlayerId),
  #[error("unit not found: {0}")]
  UnitNotFound(UnitId),
  #[error("world is full")]
  WorldIsFull,

  #[error(transparent)]
  Event(#[from] EventError),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_str())
  }
}

impl From<Error> for String {
  fn from(value: Error) -> Self {
    value.to_string()
  }
}
