#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"
cargo_toml = "1.0"

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

#![feature(iterator_try_collect)]

use anyhow::{Context, Result, anyhow};
use cargo_toml::{Manifest, Publish};
use nil_util::spawn;
use semver::Version;
use serde::Deserialize;
use std::fmt::Write;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

const REGISTRY: &str = "https://crates.io/api/v1/crates";

fn main() -> Result<()> {
  let mut command = String::from("cargo publish --workspace");

  let version = Manifest::from_path("Cargo.toml")?
    .workspace
    .and_then(|workspace| workspace.package?.version)
    .expect("failed to get workspace version");

  for entry in fs::read_dir("crates")? {
    let path = entry?.path().join("Cargo.toml");
    let manifest = Manifest::from_path(path)?;

    if let Some(package) = manifest.package
      && let Ok(publish) = package.publish.get()
      && matches!(publish, Publish::Flag(true))
    {
      let name = package.name.as_str();
      let url = format!("{REGISTRY}/{name}/versions");

      let mut response = ureq::get(&url)
        .config()
        .http_status_as_error(false)
        .build()
        .call()?;

      if !response.status().is_success() {
        Err(anyhow!("{}", response.body_mut().read_to_string()?))
          .with_context(|| format!("failed to fetch {name}"))?;
      }

      if response
        .body_mut()
        .read_json::<Crate>()?
        .versions
        .iter()
        .any(|it| it.num >= version)
      {
        write!(command, " --exclude {name}")?;
      }

      sleep(Duration::from_secs(1));
    }
  }

  spawn!(command)
}

#[derive(Deserialize)]
struct Crate {
  versions: Vec<CrateVersion>,
}

#[derive(Deserialize)]
struct CrateVersion {
  num: Version,
}
