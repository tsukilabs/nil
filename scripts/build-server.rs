#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"
futures = "0.3"
octocrab = "=0.54.0"
serde_json = "1.0"
ureq = "3.3"

[dependencies.clap]
version = "4.6"
features = ["derive"]

[dependencies.nil-util]
path = "../crates/nil-util"

[dependencies.serde]
version = "1.0"
features = ["derive"]
---

use anyhow::{Context, Result};
use clap::Parser;
use futures::executor::block_on;
use nil_util::{spawn, spawn_fmt};
use serde::Deserialize;
use serde_json::from_slice;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Parser)]
struct Args {
  #[arg(long)]
  release: bool,

  #[arg(long)]
  target_dir: Option<PathBuf>,
}

fn main() -> Result<()> {
  block_on(execute())
}

async fn execute() -> Result<()> {
  spawn!("cargo build --profile release-server --package nil-server")?;

  let args = Args::parse();
  let name = if cfg!(windows) { "nil-server.exe" } else { "nil-server" };
  let path = format!("target/release-server/{name}");

  if let Some(target_dir) = args.target_dir {
    fs::copy(&path, target_dir.join(name))?;
  }

  if args.release && cfg!(target_os = "linux") {
    let package = fs::read("package.json")?;
    let package = from_slice::<Package>(&package)?;
    let version = package.version;

    let asset_name = format!("Call.of.Nil_{version}_server");
    let asset_path = format!("target/release-server/{asset_name}");

    fs::rename(path, &asset_path)?;

    let octocrab = octocrab::instance();
    let repository = octocrab.repos("tsukilabs", "nil");
    let tag_name = repository
      .releases()
      .get_latest()
      .await?
      .tag_name;

    upload_asset(&tag_name, &asset_path)?;

    if let Ok(token) = env::var("TSUKILABS_TOKEN") {
      ureq::get("https://tsukilabs.dev.br/release/nil")
        .header("Authorization", format!("Bearer {token}"))
        .call()
        .context("failed to update remote server")?;
    }
  }

  Ok(())
}

fn upload_asset(tag_name: &str, path: &str) -> Result<()> {
  spawn_fmt!("gh release upload --clobber {tag_name} {path} -R tsukilabs/nil")
    .context("failed to upload asset")
}

#[derive(Deserialize)]
struct Package {
  version: String,
}
