// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_client::Client;
use std::sync::LazyLock;
use tokio::sync::RwLock;

pub(crate) static CLIENT: LazyLock<RwLock<Client>> = LazyLock::new(RwLock::default);
