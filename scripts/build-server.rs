#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"
serde_json = "1.0"

[dependencies.clap]
version = "4.5"
features = ["derive"]

[dependencies.nil-util]
path = "../crates/nil-util"

[dependencies.serde]
version = "1.0"
features = ["derive"]

[dependencies.ureq]
version = "3.2"
features = ["json"]
---

use anyhow::Result;
use clap::Parser;
use nil_util::{output, output_fmt, spawn, spawn_fmt};
use serde::Deserialize;
use serde_json::json;
use std::fmt::Write;
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
  spawn!("cargo build --profile release-server --package nil-server")?;

  let args = Args::parse();
  let name = if cfg!(windows) { "nil-server.exe" } else { "nil-server" };
  let path = format!("target/release-server/{name}");

  if let Some(target_dir) = args.target_dir {
    fs::copy(&path, target_dir.join(name))?;
  }

  if args.release && cfg!(target_os = "linux") {
    let package = fs::read("package.json")?;
    let package = serde_json::from_slice::<Package>(&package)?;
    let version = package.version;

    let asset_name = format!("Call.of.Nil_{version}_server");
    let asset_path = format!("target/release-server/{asset_name}");

    fs::rename(path, &asset_path)?;

    let bytes = output!("gh release view --json tagName -R tsukilabs/nil")?;
    let tag_name = serde_json::from_slice::<Release>(&bytes)?.tag_name;

    spawn_fmt!("gh release upload --clobber {tag_name} {asset_path} -R tsukilabs/nil")?;

    if let Ok(token) = env::var("TSUKILABS_TOKEN") {
      ureq::get("http://tsukilabs.dev.br/release/nil")
        .header("Authorization", format!("Bearer {token}"))
        .call()?;

      if let Ok(webhook_url) = env::var("NIL_DISCORD_WEBHOOK_URL") {
        let bytes = output_fmt!("gh release view {tag_name} --json assets -R tsukilabs/nil")?;
        let asset_url = serde_json::from_slice::<Release>(&bytes)?
          .assets
          .into_iter()
          .find_map(|asset| (asset.name == asset_name).then_some(asset.url))
          .expect("Release asset not found");

        #[rustfmt::skip]
        let write_webhook_content = |message: &mut String| -> Result<()> {
          writeln!(message, "Server updated to v{version}")?;
          writeln!(message, "<https://github.com/tsukilabs/nil/releases/tag/{tag_name}>")?;
          writeln!(message, "<{asset_url}>")?;
          Ok(())
        };

        let mut message = String::new();
        write_webhook_content(&mut message)?;

        ureq::post(webhook_url).send_json(json!({ "content": message }))?;
      }
    }
  }

  Ok(())
}

#[derive(Deserialize)]
struct Package {
  version: String,
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Release {
  tag_name: String,
  assets: Vec<Asset>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Asset {
  name: String,
  url: String,
}
