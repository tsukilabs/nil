// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::player::{PlayerId, PlayerOptions, PlayerStatus};
use nil_core::world::config::WorldId;
use nil_crypto::password::Password;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GetPlayerRequest {
  pub world: WorldId,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GetPlayerCoordsRequest {
  pub world: WorldId,
  pub id: PlayerId,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GetPlayerIdsRequest {
  pub world: WorldId,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GetPlayerMaintenanceRequest {
  pub world: WorldId,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GetPlayerMilitaryRequest {
  pub world: WorldId,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GetPlayerReportsRequest {
  pub world: WorldId,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GetPlayerStatusRequest {
  pub world: WorldId,
  pub id: PlayerId,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GetPlayerStorageCapacityRequest {
  pub world: WorldId,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GetPlayerWorldsRequest {
  pub id: PlayerId,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GetPublicPlayerRequest {
  pub world: WorldId,
  pub id: PlayerId,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GetPublicPlayersRequest {
  pub world: WorldId,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PlayerExistsRequest {
  pub world: WorldId,
  pub id: PlayerId,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct SetPlayerStatusRequest {
  pub world: WorldId,
  pub status: PlayerStatus,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, optional_fields = nullable)]
pub struct SpawnPlayerRequest {
  pub world: WorldId,
  #[serde(default)]
  pub world_password: Option<Password>,
  pub options: PlayerOptions,
}
