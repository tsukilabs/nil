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
  script: String,

  #[arg(short = 'r', long)]
  release: bool,
}

fn main() -> Result<()> {
  let args = Args::parse();
  let mut command = if args.release {
    String::from("nls ")
  } else {
    String::from("cargo run -p nil-lua-cli -- ")
  };

  let script = args.script.trim_end_matches(".lua");
  write!(command, " crates/nil-lua/examples/{script}.lua")?;

  spawn!(command)
}
