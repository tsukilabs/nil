// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::Builder;
use nil_core::resources::Resources;
use nil_core::ruler::Ruler;
use nil_core::world::config::WorldId;
use nil_payload_macros::FromWorld;
use serde::{Deserialize, Serialize};

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct BuyResourcesRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub resources: Resources,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize, FromWorld)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetMarketRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize, FromWorld)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetMarketFeeRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct SellResourcesRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub resources: Resources,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct SendResourcesRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub recipient: Ruler,
  #[builder(into)]
  pub resources: Resources,
}
