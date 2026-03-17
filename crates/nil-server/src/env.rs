// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use std::env;
use std::num::NonZeroU16;

pub fn remote_world_limit() -> NonZeroU16 {
  env::var("NIL_REMOTE_WORLD_LIMIT")
    .ok()
    .and_then(|it| it.parse::<NonZeroU16>().ok())
    .unwrap_or_else(|| unsafe { NonZeroU16::new_unchecked(100) })
}

pub fn remote_world_limit_per_user() -> NonZeroU16 {
  env::var("NIL_REMOTE_WORLD_LIMIT_PER_USER")
    .ok()
    .and_then(|it| it.parse::<NonZeroU16>().ok())
    .unwrap_or_else(|| unsafe { NonZeroU16::new_unchecked(3) })
}
