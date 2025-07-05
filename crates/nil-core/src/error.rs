// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::infrastructure::building::{BuildingId, BuildingLevel};
use crate::infrastructure::mine::MineId;
use crate::infrastructure::storage::StorageId;
use crate::player::PlayerId;
use crate::script::ScriptId;
use crate::village::Coord;
use serde::Serialize;
use serde::ser::Serializer;
use std::result::Result as StdResult;
use strum::EnumIs;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug, EnumIs, thiserror::Error)]
#[remain::sorted]
pub enum Error {
  #[error("No stats found for building \"{0}\"")]
  BuildingStatsNotFound(BuildingId),

  #[error("No stats found for building \"{0}\" at level {1}")]
  BuildingStatsNotFoundForLevel(BuildingId, BuildingLevel),

  #[error("Building \"{0}\" is already at its minimum level")]
  CannotDecreaseBuildingLevel(BuildingId),

  #[error("Building \"{0}\" is already at its maximum level")]
  CannotIncreaseBuildingLevel(BuildingId),

  #[error("Coord out of bounds: {0}")]
  CoordOutOfBounds(Coord),

  #[error("Failed to load world")]
  FailedToLoadWorld,

  #[error("Failed to save world")]
  FailedToSaveWorld,

  #[error("Index out of bounds: {0}")]
  IndexOutOfBounds(usize),

  #[error("Insufficient resources")]
  InsufficientResources,

  #[error("No stats found for mine \"{0}\"")]
  MineStatsNotFound(MineId),

  #[error("No stats found for mine \"{0}\" at level {1}")]
  MineStatsNotFoundForLevel(MineId, BuildingLevel),

  #[error("No players in the world")]
  NoPlayer,

  #[error("Player \"{0}\" is not a guest")]
  NotAGuest(PlayerId),

  #[error("Player already spawned: {0}")]
  PlayerAlreadySpawned(PlayerId),

  #[error("Player \"{0}\" has already taken their turn")]
  PlayerIsNotPending(PlayerId),

  #[error("Player not found: {0}")]
  PlayerNotFound(PlayerId),

  #[error("Round already started")]
  RoundAlreadyStarted,

  #[error("Round has pending players")]
  RoundHasPendingPlayers,

  #[error("Round has not started yet")]
  RoundNotStarted,

  #[error("Script not found: {0}")]
  ScriptNotFound(ScriptId),

  #[error("No stats found for storage \"{0}\"")]
  StorageStatsNotFound(StorageId),

  #[error("No stats found for storage \"{0}\" at level {1}")]
  StorageStatsNotFoundForLevel(StorageId, BuildingLevel),

  #[error("Village not found: {0}")]
  VillageNotFound(Coord),

  #[error("World is full")]
  WorldIsFull,
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_str())
  }
}

pub trait WrapOk<T> {
  fn wrap_ok(self) -> Result<T>;
}

impl<T> WrapOk<T> for T {
  fn wrap_ok(self) -> Result<T> {
    Ok(self)
  }
}
