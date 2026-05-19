#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"

[dependencies.clap]
version = "4.6"
features = ["derive"]

[dependencies.nil-util]
path = "../crates/nil-util"
---

use anyhow::Result;
use clap::Parser;
use nil_util::{spawn, spawn_fmt};

#[derive(Parser)]
struct Args {
  #[arg(long)]
  ask: bool,

  #[arg(long)]
  major: bool,

  #[arg(long)]
  minor: bool,
}

fn main() -> Result<()> {
  let args = Args::parse();

  let command = if args.major {
    "miho bump major"
  } else if args.minor {
    "miho bump minor"
  } else {
    "miho bump"
  };

  if args.ask {
    spawn!(command)?;
  } else {
    spawn_fmt!("{command} -k")?;
  }

  spawn!("pnpm run format")?;

  Ok(())
}
