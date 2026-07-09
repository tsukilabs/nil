// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::coord::Coord;
use crate::decl_recruit_queue;
use crate::error::{Error, Result};
use crate::infrastructure::queue::{InfrastructureQueue, InfrastructureQueueOrder};
use crate::military::squad::Squad;
use crate::military::squad::size::SquadSize;
use crate::military::unit::{UnitBox, WorkshopUnitId};
use crate::resources::Resources;
use crate::resources::workforce::Workforce;
use bon::Builder;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::num::NonZeroU32;
use strum::EnumIs;
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct WorkshopRecruitQueue {
  #[cfg_attr(feature = "typescript", ts(as = "Vec<WorkshopRecruitOrder>"))]
  orders: VecDeque<WorkshopRecruitOrder>,
}

decl_recruit_queue!(Workshop);
