#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"
itertools = "0.14"
serde_json = "1.0"
tap = "=1.0.1"
toml = "1.1"

[dependencies.nil-util]
path = "../crates/nil-util"

[dependencies.semver]
version = "1.0"
features = ["serde"]

[dependencies.serde]
version = "1.0"
features = ["derive"]

[dependencies.ureq]
version = "3.3"
features = ["json"]
---

use anyhow::{Result, anyhow};
use itertools::Itertools;
use nil_util::spawn;
use semver::Version;
use serde_json::Value as JsonValue;
use std::fmt::Write;
use std::fs;
use std::thread::sleep;
use std::time::Duration;
use tap::Pipe;
use toml::Value as TomlValue;

const REGISTRY: &str = "https://crates.io/api/v1/crates";

fn main() -> Result<()> {
  let mut command = String::from("cargo publish --workspace");
  let version = fs::read("Cargo.toml")?
    .pipe(|bytes| toml::from_slice::<TomlValue>(&bytes))?
    .get("workspace")
    .and_then(|workspace| workspace.get("package"))
    .and_then(|package| package.get("version"))
    .and_then(TomlValue::as_str)
    .ok_or_else(|| anyhow!("failed to get workspace version"))?
    .parse::<Version>()?;

  for krate in fs::read_dir("crates")? {
    let krate = krate?;
    if let Some(name) = krate.file_name().to_str() {
      sleep(Duration::from_millis(100));
      let url = format!("{REGISTRY}/{name}/versions");

      if let Ok(mut response) = ureq::get(&url).call() {
        let versions: Vec<Version> = response
          .body_mut()
          .read_json::<JsonValue>()?
          .get("versions")
          .and_then(JsonValue::as_array)
          .ok_or_else(|| anyhow!("failed to get versions for crate {name}"))?
          .iter()
          .filter_map(|version| {
            version
              .get("num")
              .and_then(JsonValue::as_str)
          })
          .map(Version::parse)
          .try_collect()?;

        if versions.iter().any(|it| it >= &version) {
          write!(command, " --exclude {name}")?;
        }
      }
    }
  }

  spawn!(command)
}
