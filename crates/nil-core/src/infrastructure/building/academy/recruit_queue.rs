// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::Coord;
use crate::decl_recruit_queue;
use crate::error::{Error, Result};
use crate::infrastructure::queue::{InfrastructureQueue, InfrastructureQueueOrder};
use crate::military::squad::{Squad, SquadSize};
use crate::military::unit::{AcademyUnitId, UnitBox};
use crate::resources::Resources;
use crate::resources::workforce::Workforce;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::num::NonZeroU32;
use strum::EnumIs;
use uuid::Uuid;

decl_recruit_queue!(Academy);
