// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::continent::Coord;
use nil_core::infrastructure::building::academy::recruit_queue::AcademyRecruitQueue;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[cfg(feature = "axum")]
use nil_payload_macros::IntoJsonResponse;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct CheatGetAcademyRecruitQueueResponse(pub AcademyRecruitQueue);

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct CheatGetAcademyRecruitQueuesResponse(pub Vec<(Coord, AcademyRecruitQueue)>);
