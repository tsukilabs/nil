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
  #[arg(short = 'a', long)]
  allow: Vec<String>,
  #[arg(short = 'u', long)]
  unused: bool,
}

fn main() -> Result<()> {
  let mut args = Args::parse();
  let mut command = String::from("cargo clippy --workspace --");

  if args.unused {
    args.allow.push("unused".to_owned());
  }

  for lint in args.allow {
    write!(command, " -A {lint}")?;
  }

  spawn!(command)
}
