#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"

[dependencies.clap]
version = "4.6"
features = ["derive"]
---

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args;

fn main() -> Result<()> {
  Ok(())
}
