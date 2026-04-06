// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![feature(try_blocks_heterogeneous)]

use anyhow::Result;
use bytesize::ByteSize;
use mimalloc::MiMalloc;
use nil_log::{Directives, Layers};
use nil_server::remote;
use std::env;
use std::time::Duration;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};
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

macro_rules! b {
  ($bytes:expr) => {{ ByteSize::b($bytes).display().si() }};
}

fn watch_process(pid: Pid) {
  let mut sys = System::new_with_specifics(
    RefreshKind::nothing().with_processes(
      ProcessRefreshKind::nothing()
        .with_cpu()
        .with_disk_usage()
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
          .with_disk_usage()
          .with_memory(),
      );

      if let Some(process) = sys.process(pid) {
        let runtime = Handle::current();
        let metrics = runtime.metrics();
        let disk_usage = process.disk_usage();

        tracing::info!(
          name = %process.name().to_string_lossy(),
          cpu_usage = format!("{:.2}%", process.cpu_usage()),
          memory = %b!(process.memory()),
          virtual_memory = %b!(process.virtual_memory()),
          read_bytes = %b!(disk_usage.read_bytes),
          total_read_bytes = %b!(disk_usage.total_read_bytes),
          written_bytes = %b!(disk_usage.written_bytes),
          total_written_bytes = %b!(disk_usage.total_written_bytes),
          run_time = ?Duration::from_secs(process.run_time()),
          tokio_global_queue_depth = metrics.global_queue_depth(),
          tokio_num_alive_tasks = metrics.num_alive_tasks(),
          tokio_num_workers = metrics.num_workers(),
        );
      }

      let mins = rand::random_range(10..=30);
      sleep(Duration::from_mins(mins)).await;
    }
  });
}
