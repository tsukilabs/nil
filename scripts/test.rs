#!/usr/bin/env -S cargo +nightly -Zscript
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
  miri: bool,
}

fn main() -> Result<()> {
  let args = Args::parse();
  let mut command = if args.miri {
    spawn!("rustup component add --toolchain nightly miri")?;
    String::from("cargo miri test")
  } else {
    String::from("cargo test")
  };

  write!(command, " --exclude nil --workspace --tests")?;

  spawn!(command)
}
