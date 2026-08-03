// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::Builder;
use nil_core::continent::coord::Coord;
use nil_core::military::army::ArmyId;
use nil_core::military::maneuver::{ManeuverId, ManeuverRequest};
use nil_core::world::config::WorldId;
use nil_payload_macros::FromWorld;
use serde::{Deserialize, Serialize};

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CancelManeuverRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  pub id: ManeuverId,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize, FromWorld)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetArmiesRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetArmyRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  pub id: ArmyId,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetArmyOwnerRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  pub id: ArmyId,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetIdleArmiesAtRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  pub coord: Coord,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize, FromWorld)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetIdleArmiesCoordsRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetManeuverRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  pub id: ManeuverId,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct RequestManeuverRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  pub request: ManeuverRequest,
}
