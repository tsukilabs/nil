#!/usr/bin/env -S cargo +nightly -Zscript
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
  spawn!("cargo clean --verbose")?;

  if fs::exists("app/src-tauri/target")? {
    fs::remove_dir_all("app/src-tauri/target")?;
  }

  Ok(())
}
