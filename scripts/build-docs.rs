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
  spawn!("pnpm install --no-frozen-lockfile")?;
  spawn!("pnpm run -F docs build")?;

  fs::write("docs/dist/.nojekyll", "")?;
  fs::write("docs/dist/CNAME", "nil.dev.br")?;

  Ok(())
}
