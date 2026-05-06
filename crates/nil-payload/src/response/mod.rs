// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod battle;
pub mod chat;
pub mod cheat;
pub mod city;
pub mod continent;
pub mod infrastructure;
pub mod military;
pub mod npc;
pub mod player;
pub mod ranking;
pub mod report;
pub mod round;
pub mod user;
pub mod world;

use nil_core::player::PlayerId;
use nil_server_types::ServerKind;
use nil_server_types::auth::Token;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[cfg(feature = "axum")]
use nil_payload_macros::IntoJsonResponse;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct AuthorizeResponse(pub Token);

#[derive(Clone, Copy, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct GetServerKindResponse(pub ServerKind);

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct ValidateTokenResponse(pub Option<PlayerId>);
