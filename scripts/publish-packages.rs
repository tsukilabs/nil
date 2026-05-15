#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"
serde_json = "1.0"

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

use anyhow::Result;
use nil_util::spawn_fmt;
use semver::Version;
use serde::Deserialize;
use serde_json::{Value as Json, from_slice};
use std::collections::HashMap;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
  let mut packages = HashMap::new();
  for entry in fs::read_dir("packages")? {
    let entry = entry?;
    if entry.file_type()?.is_dir()
      && let Some(name) = entry
        .file_name()
        .to_str()
        .map(ToOwned::to_owned)
    {
      let path = entry.path().join("package.json");
      let bytes = fs::read(path)?;
      let manifest: Package = from_slice(&bytes)?;

      if !manifest.private {
        packages.insert(name, manifest);
      }
    }
  }

  for (dir, package) in packages {
    if package.scripts.contains_key("build") {
      spawn_fmt!("pnpm run -F {} build", package.name)?;
    }

    let url = format!("https://registry.npmjs.org/{}", package.name);
    let body = ureq::get(url)
      .header("Accept", "application/vnd.npm.install-v1+json")
      .call()?
      .body_mut()
      .read_to_vec()?;

    if from_slice::<Metadata>(&body)?
      .versions
      .keys()
      .all(|it| it < &package.version)
    {
      let path = format!("packages/{dir}");
      spawn_fmt!("npm publish {path} --access=public --tag=latest --provenance")?;
    }

    sleep(Duration::from_millis(200));
  }

  Ok(())
}

#[derive(Deserialize)]
struct Package {
  name: String,
  version: Version,
  private: bool,

  #[serde(default)]
  scripts: HashMap<String, Json>,
}

#[derive(Deserialize)]
struct Metadata {
  versions: HashMap<Version, Json>,
}
