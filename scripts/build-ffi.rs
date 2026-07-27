---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"
octocrab = "=0.54.1"
serde_json = "1.0"

[dependencies.clap]
version = "4.6"
features = ["derive"]

[dependencies.nil-util]
path = "../crates/nil-util"

[dependencies.serde]
version = "1.0"
features = ["derive"]

[dependencies.tokio]
version = "1.53"
features = ["full"]
---

use anyhow::{Context, Result};
use clap::Parser;
use nil_util::{spawn, spawn_fmt};
use octocrab::Octocrab;
use serde::Deserialize;
use serde_json::from_slice;
use std::env::var;
use std::fs;

#[derive(Parser)]
struct Args {
  #[arg(long)]
  publish: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
  spawn!("cargo build --profile release-ffi --package nil-ffi")?;
  spawn!("pnpm run -F @tsukilabs/nil-ffi build")?;

  let args = Args::parse();
  let ext = if cfg!(windows) { ".dll" } else { ".so" };
  let path = format!("target/release-ffi/nil_ffi{ext}");

  if args.publish {
    let package = fs::read("package.json")?;
    let package = from_slice::<Package>(&package)?;
    let version = package.version;

    let asset_name = format!("libcallofnil_{version}{ext}");
    let asset_path = format!("target/release-ffi/{asset_name}");

    fs::rename(path, &asset_path)?;

    let octocrab = Octocrab::builder()
      .personal_token(var("GH_TOKEN")?)
      .build()?;

    let repository = octocrab.repos("tsukilabs", "nil");
    let tag_name = repository
      .releases()
      .get_latest()
      .await?
      .tag_name;

    upload_asset(&tag_name, &asset_path)?;
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
