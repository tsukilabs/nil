#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"

[dependencies.clap]
version = "4.6"
features = ["derive"]

[dependencies.nil-env]
path = "../crates/nil-env"

[dependencies.nil-util]
path = "../crates/nil-util"
---

use anyhow::Result;
use clap::Parser;
use nil_env::Var;
use nil_util::spawn;
use std::fmt::Write;

#[derive(Parser)]
struct Args {
  #[arg(long)]
  android: bool,

  #[arg(long)]
  device: Option<String>,

  #[arg(long)]
  local: bool,

  #[arg(long)]
  verbose: bool,
}

fn main() -> Result<()> {
  let args = Args::parse();
  let mut env = Vec::new();

  if args.local && !args.android {
    env.push((Var::RemoteServerAddr, "http://127.0.0.1:3000/"));
  }

  if args.verbose {
    env.push((Var::LogLevel, "trace"));

    if !args.android {
      env.push((Var::LogTowerHttp, "true"));
    }
  } else {
    env.push((Var::LogLevel, "debug"));
  }

  if args.android {
    let mut command = String::from("cargo tauri android dev");
    if let Some(device) = args.device {
      write!(command, " {device}")?;
    }

    spawn!(command)
  } else {
    spawn!("cargo tauri dev", env)
  }
}
