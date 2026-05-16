// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::{Deref, DerefMut, From, Into};
use nil_core::player::PlayerId;
use nil_server_types::auth::Token;
use serde::{Deserialize, Serialize};

#[cfg(feature = "axum")]
use nil_payload_macros::IntoJsonResponse;

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Clone, Debug, Deref, DerefMut, From, Into, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct AuthorizeResponse(pub Token);

#[derive(Clone, Debug, Deref, DerefMut, From, Into, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct ValidateTokenResponse(pub Option<PlayerId>);
