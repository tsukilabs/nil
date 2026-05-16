// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(
  const_clone,
  const_cmp,
  const_convert,
  const_default,
  const_result_trait_fn,
  const_trait_impl,
  derive_const,
  str_as_str
)]

pub mod auth;
pub mod round;
pub mod world;

use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};
use strum::EnumIs;

#[derive(Clone, Copy, Debug, EnumIs, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum ServerKind {
  Local { id: WorldId },
  Remote,
}
