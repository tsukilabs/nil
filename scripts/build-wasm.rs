#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"
globset = "0.4"
serde_json = "1.0"
ureq = "3.2"
walkdir = "2.5"

[dependencies.clap]
version = "4.5"
features = ["derive"]

[dependencies.nil-util]
path = "../crates/nil-util"

[dependencies.serde]
version = "1.0"
features = ["derive"]
---

use anyhow::Result;
use clap::Parser;
use globset::{Glob, GlobBuilder, GlobSetBuilder};
use nil_util::{spawn, spawn_fmt};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser)]
struct Args {
  #[arg(long)]
  install: bool,
  #[arg(long)]
  publish: bool,
  #[arg(long)]
  release: bool,
}

fn main() -> Result<()> {
  let args = Args::parse();
  if args.install {
    spawn!("rustup target add wasm32-unknown-unknown")?;
    spawn!("cargo install wasm-bindgen-cli")?;
    spawn!("cargo install wasm-pack")?;
  }

  let krates = ["nil-continent"];

  for krate in krates {
    let path = format!("crates/{krate}/pkg");
    if fs::exists(&path)? {
      fs::remove_dir_all(&path)?;
    }

    let mut command = format!("wasm-pack build crates/{krate} --scope tsukilabs");
    if args.release || args.publish {
      write!(command, " --profile release-wasm")?;
    } else {
      write!(command, " --dev --no-opt")?;
    }

    spawn!(command)?;
  }

  if args.publish {
    let package = fs::read("package.json")?;
    let package = serde_json::from_slice::<Package>(&package)?;
    let current_version = package.version;

    let globset = GlobSetBuilder::new()
      .add(glob("*.js"))
      .add(glob("*.ts"))
      .build()
      .unwrap();

    spawn!("pnpm install --no-frozen-lockfile")?;

    for krate in krates {
      let url = format!("https://registry.npmjs.org/@tsukilabs/{krate}");
      let body = ureq::get(url)
        .header("Accept", "application/vnd.npm.install-v1+json")
        .call()?
        .body_mut()
        .read_to_vec()?;

      if !serde_json::from_slice::<Metadata>(&body)?
        .versions
        .contains_key(&current_version)
      {
        let path = format!("crates/{krate}/pkg");
        let files = WalkDir::new(&path)
          .into_iter()
          .flatten()
          .map(DirEntry::into_path)
          .filter(|path| path.is_file() && globset.is_match(path))
          .collect::<Vec<PathBuf>>();

        for file in files {
          let mut contents = String::new();
          writeln!(contents, "// Copyright (C) Call of Nil contributors")?;
          writeln!(contents, "// SPDX-License-Identifier: AGPL-3.0-only")?;
          writeln!(contents, "\n{}", fs::read_to_string(&file)?)?;

          fs::write(file, contents)?;
        }

        spawn_fmt!("npm publish {path} --access=public --tag=latest --provenance")?;
      }
    }
  }

  Ok(())
}

#[derive(Deserialize)]
struct Package {
  version: String,
}

#[derive(Deserialize)]
struct Metadata {
  versions: HashMap<String, Value>,
}

fn glob(glob: &str) -> Glob {
  GlobBuilder::new(glob)
    .case_insensitive(true)
    .build()
    .unwrap()
}
