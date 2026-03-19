// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![feature(try_blocks_heterogeneous)]

use anyhow::Result;
use mimalloc::MiMalloc;
use nil_log::{Directives, Layers};
use nil_server::remote;
use std::env;
use std::time::Duration;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};
use tokio::task::spawn;
use tokio::time::sleep;

#[global_allocator]
static ALLOCATOR: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
  let _guard = nil_log::setup()
    .directives(Directives::all())
    .debug_layers(Layers::STDERR)
    .release_layers(Layers::FILE)
    .call()?;

  if let Ok(pid) = sysinfo::get_current_pid() {
    watch_process(pid);
  }

  let result = try bikeshed Result<()> {
    let database_url = env::var("NIL_DATABASE_URL")?;
    remote::start(database_url).await?;
  };

  if let Err(err) = &result {
    tracing::error!(message = %err, error = ?err);
  }

  result
}

fn watch_process(pid: Pid) {
  spawn(async move {
    let mut sys = System::new_with_specifics(
      RefreshKind::nothing().with_processes(
        ProcessRefreshKind::nothing()
          .with_cpu()
          .with_disk_usage()
          .with_memory(),
      ),
    );

    loop {
      sys.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid]),
        true,
        ProcessRefreshKind::nothing()
          .with_cpu()
          .with_disk_usage()
          .with_memory(),
      );

      if let Some(process) = sys.process(pid) {
        tracing::info!(
          name = %process.name().to_string_lossy(),
          cpu_usage = process.cpu_usage(),
          disk_usage = ?process.disk_usage(),
          memory = process.memory(),
          virtual_memory = process.virtual_memory(),
          run_time = ?Duration::from_secs(process.run_time()),
        );
      }

      sleep(Duration::from_mins(30)).await;
    }
  });
}
