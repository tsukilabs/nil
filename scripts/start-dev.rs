#!/usr/bin/env cargo
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"

[dependencies.clap]
version = "4.5"
features = ["derive"]

[dependencies.nil-util]
path = "../crates/nil-util"
---

use anyhow::Result;
use clap::Parser;
use nil_util::spawn;
use std::fmt::Write;

#[derive(Parser)]
struct Args {
  #[arg(long)]
  android: bool,
  #[arg(long)]
  device: Option<String>,
  #[arg(long)]
  remote: bool,
  #[arg(long)]
  verbose: bool,
  #[arg(long)]
  wasm: bool,
}

fn main() -> Result<()> {
  let args = Args::parse();
  if args.wasm {
    spawn!("pnpm run wasm")?;
  }

  let mut env = Vec::new();

  if args.remote && !args.android {
    env.push(("NIL_REMOTE_SERVER_ADDR", "tsukilabs.dev.br/nil"));
  }

  if args.verbose {
    env.push(("NIL_LOG_TOWER_HTTP", "true"));
  }

  if args.android {
    let mut command = String::from("cargo tauri android dev");
    if let Some(device) = args.device {
      write!(command, " {device}")?;
    }

    spawn!(command, env)
  } else {
    spawn!("cargo tauri dev", env)
  }
}
