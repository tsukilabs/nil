// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![feature(str_as_str, try_blocks_heterogeneous)]

use anyhow::{Context as _, Result, anyhow};
use bytesize::ByteSize;
use humantime::format_duration;
use mimalloc::MiMalloc;
use nil_log::{Directives, Layers};
use nil_server::remote;
use std::time::Duration;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};
use tokio::runtime::Handle;
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

  watch_process()?;

  let result = try bikeshed Result<()> {
    let database_url = nil_env::database_url()?;
    remote::start(database_url.as_str()).await?;
  };

  if let Err(err) = &result {
    tracing::error!(message = %err, error = ?err);
  }

  result
}

macro_rules! b {
  ($bytes:expr) => {{ ByteSize::b($bytes).display().si() }};
}

fn watch_process() -> Result<()> {
  let pid = sysinfo::get_current_pid()
    .map_err(|err| anyhow!("{err}"))
    .context("Failed to get process pid")?;

  let mut sys = System::new_with_specifics(
    RefreshKind::nothing().with_processes(
      ProcessRefreshKind::nothing()
        .with_cpu()
        .with_memory(),
    ),
  );

  spawn(async move {
    loop {
      sys.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid]),
        true,
        ProcessRefreshKind::nothing()
          .with_cpu()
          .with_memory(),
      );

      if let Some(process) = sys.process(pid) {
        let runtime = Handle::current();
        let metrics = runtime.metrics();

        let run_time = Duration::from_secs(process.run_time());

        tracing::info!(
          name = %process.name().to_string_lossy(),
          cpu_usage = format!("{:.2}%", process.cpu_usage()),
          memory = %b!(process.memory()),
          run_time = %format_duration(run_time),
          tokio_num_alive_tasks = metrics.num_alive_tasks(),
          tokio_num_workers = metrics.num_workers(),
        );
      }

      sleep(Duration::from_mins(10)).await;
    }
  });

  Ok(())
}
