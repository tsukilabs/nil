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
use nil_util::spawn;

#[derive(Parser)]
struct Args {
  #[arg(long)]
  major: bool,

  #[arg(long)]
  minor: bool,
}

fn main() -> Result<()> {
  let args = Args::parse();

  if args.major {
    spawn!("miho bump major -k")?;
  } else if args.minor {
    spawn!("miho bump minor -k")?;
  } else {
    spawn!("miho bump -k")?;
  }

  spawn!("pnpm run format")?;

  Ok(())
}
