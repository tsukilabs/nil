// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use std::sync::LazyLock;
use tokio::runtime::{Builder, Runtime};

pub(crate) static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
  Builder::new_multi_thread()
    .enable_all()
    .thread_name("libnil-worker")
    .build()
    .expect("failed to initialize tokio runtime")
});
