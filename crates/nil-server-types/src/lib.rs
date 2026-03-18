// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(str_as_str)]

pub mod auth;
pub mod round;
pub mod time;
pub mod world;

use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};
use strum::EnumIs;

#[derive(Clone, Copy, Debug, EnumIs, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ServerKind {
  Local { id: WorldId },
  Remote,
}
