// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::Coord;
use crate::decl_world_recruit_order_fn;
use crate::error::{Error, Result};
use crate::infrastructure::building::academy::recruit_queue::{
  AcademyRecruitOrderId,
  AcademyRecruitOrderRequest,
};
use crate::world::World;

decl_world_recruit_order_fn!(
  add_academy_recruit_order,
  cancel_academy_recruit_order,
  AcademyRecruitOrderId,
  AcademyRecruitOrderRequest
);
