#!/usr/bin/env cargo
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"

[dependencies.nil-util]
path = "../crates/nil-util"
---

use anyhow::Result;
use nil_util::spawn;
use std::fs;

fn main() -> Result<()> {
  spawn!("cargo run -p nil-server")
}
