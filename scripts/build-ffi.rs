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

fn main() -> Result<()> {
  spawn!("cargo build --profile release-ffi --package nil-ffi")
}
