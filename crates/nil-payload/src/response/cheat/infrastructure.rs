// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::continent::Coord;
use nil_core::infrastructure::Infrastructure;
use nil_core::infrastructure::building::r#impl::academy::recruit_queue::AcademyRecruitQueue;
use nil_core::infrastructure::building::r#impl::prefecture::build_queue::PrefectureBuildQueue;
use nil_core::infrastructure::building::r#impl::stable::recruit_queue::StableRecruitQueue;
use nil_core::infrastructure::storage::OverallStorageCapacity;
use serde::{Deserialize, Serialize};

#[cfg(feature = "axum")]
use nil_payload_macros::IntoJsonResponse;

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetAcademyRecruitQueueResponse(pub AcademyRecruitQueue);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetAcademyRecruitQueuesResponse(pub Vec<(Coord, AcademyRecruitQueue)>);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetAllAcademyRecruitQueuesResponse(pub Vec<(Coord, AcademyRecruitQueue)>);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetAllPrefectureBuildQueuesResponse(pub Vec<(Coord, PrefectureBuildQueue)>);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetAllStableRecruitQueuesResponse(pub Vec<(Coord, StableRecruitQueue)>);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetInfrastructureResponse(pub Infrastructure);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetPrefectureBuildQueueResponse(pub PrefectureBuildQueue);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetPrefectureBuildQueuesResponse(pub Vec<(Coord, PrefectureBuildQueue)>);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetStableRecruitQueueResponse(pub StableRecruitQueue);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetStableRecruitQueuesResponse(pub Vec<(Coord, StableRecruitQueue)>);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetStorageCapacityResponse(pub OverallStorageCapacity);
