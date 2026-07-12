#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"
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

[dependencies.tokio]
version = "1.52"
features = ["full"]
---

use anyhow::{Context, Result};
use clap::Parser;
use nil_util::{spawn, spawn_fmt};
use octocrab::Octocrab;
use serde::Deserialize;
use serde_json::from_slice;
use std::env::var;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Parser)]
struct Args {
  #[arg(long)]
  release: bool,

  #[arg(long)]
  target_dir: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
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

    let notes = repository
      .releases()
      .generate_release_notes(&tag_name)
      .send()
      .await?
      .body;

    edit_release_notes(&tag_name, &notes)?;

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

fn edit_release_notes(tag_name: &str, notes: &str) -> Result<()> {
  let notes = format!(
    "This is an early preview. The game is still under development and not yet ready to play.\n\n{notes}"
  );

  spawn_fmt!(r#"gh release edit {tag_name} --notes "{notes}" --verify-tag -R tsukilabs/nil"#)
    .context("failed to edit release notes")
}

#[derive(Deserialize)]
struct Package {
  version: String,
}
