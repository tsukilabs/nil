// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::{ContinentIndex, Coord};
use crate::infrastructure::building::{BuildingId, BuildingLevel, MineId, StorageId};
use crate::military::army::ArmyId;
use crate::military::maneuver::ManeuverId;
use crate::military::unit::UnitId;
use crate::npc::bot::BotId;
use crate::npc::precursor::PrecursorId;
use crate::player::PlayerId;
use serde::Serialize;
use serde::ser::Serializer;
use std::result::Result as StdResult;
use strum::EnumIs;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Clone, Debug, EnumIs, thiserror::Error)]
#[remain::sorted]
pub enum Error {
  #[error("Army not found")]
  ArmyNotFound(ArmyId),

  #[error("Army is not idle")]
  ArmyNotIdle(ArmyId),

  #[error("Bot already spawned: {0}")]
  BotAlreadySpawned(BotId),

  #[error("Bot not found: {0}")]
  BotNotFound(BotId),

  #[error("No stats found for building \"{0}\"")]
  BuildingStatsNotFound(BuildingId),

  #[error("No stats found for building \"{0}\" at level {1}")]
  BuildingStatsNotFoundForLevel(BuildingId, BuildingLevel),

  #[error("Building \"{0}\" is already at its minimum level")]
  CannotDecreaseBuildingLevel(BuildingId),

  #[error("Building \"{0}\" is already at its maximum level")]
  CannotIncreaseBuildingLevel(BuildingId),

  #[error("Cheating is not allowed in this world")]
  CheatingNotAllowed,

  #[error("City not found: {0}")]
  CityNotFound(Coord),

  #[error("Failed to read savedata file")]
  FailedToReadSavedata,

  #[error("Failed to write savedata file")]
  FailedToWriteSavedata,

  #[error("Not authorized to execute this action")]
  Forbidden,

  #[error("Index out of bounds: {0}")]
  IndexOutOfBounds(ContinentIndex),

  #[error("Insufficient resources")]
  InsufficientResources,

  #[error("Insufficient units")]
  InsufficientUnits,

  #[error("Maneuver not found")]
  ManeuverNotFound(ManeuverId),

  #[error("No stats found for mine \"{0}\"")]
  MineStatsNotFound(MineId),

  #[error("No stats found for mine \"{0}\" at level {1}")]
  MineStatsNotFoundForLevel(MineId, BuildingLevel),

  #[error("No players in the world")]
  NoPlayer,

  #[error("Origin and destination have the same coords")]
  OriginIsDestination(Coord),

  #[error("Player already spawned: {0}")]
  PlayerAlreadySpawned(PlayerId),

  #[error("Player \"{0}\" has already taken their turn")]
  PlayerIsNotPending(PlayerId),

  #[error("Player not found: {0}")]
  PlayerNotFound(PlayerId),

  #[error("Precursor not found: {0}")]
  PrecursorNotFound(PrecursorId),

  #[error("Round already started")]
  RoundAlreadyStarted,

  #[error("Round has pending players")]
  RoundHasPendingPlayers,

  #[error("Round has not started yet")]
  RoundNotStarted,

  #[error("No stats found for storage \"{0}\"")]
  StorageStatsNotFound(StorageId),

  #[error("No stats found for storage \"{0}\" at level {1}")]
  StorageStatsNotFoundForLevel(StorageId, BuildingLevel),

  #[error("Expected \"{0}\", got \"{1}\"")]
  UnexpectedUnit(UnitId, UnitId),

  #[error("No stats found for wall at level {0}")]
  WallStatsNotFoundForLevel(BuildingLevel),

  #[error("World is full")]
  WorldIsFull,
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_str())
  }
}
