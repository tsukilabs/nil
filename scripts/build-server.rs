#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"
serde_json = "1.0"

[dependencies.clap]
version = "4.6"
features = ["derive"]

[dependencies.nil-util]
path = "../crates/nil-util"

[dependencies.serde]
version = "1.0"
features = ["derive"]

[dependencies.ureq]
version = "3.3"
features = ["json"]
---

#![feature(try_blocks_heterogeneous)]

use anyhow::{Context, Result, anyhow};
use clap::Parser;
use nil_util::{output_fmt, spawn, spawn_fmt};
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

    let tag_name = view_release(None)?.tag_name;
    upload_asset(&tag_name, &asset_path)?;

    if let Ok(token) = env::var("TSUKILABS_TOKEN") {
      ureq::get("https://tsukilabs.dev.br/release/nil")
        .header("Authorization", format!("Bearer {token}"))
        .call()
        .context("failed to update remote server")?;

      let release = view_release(Some(&tag_name))?;

      if let Ok(webhook_url) = env::var("NIL_DISCORD_WEBHOOK_URL_PRIVATE") {
        let asset_url = release
          .assets
          .iter()
          .find_map(|asset| (asset.name == asset_name).then(|| asset.url.clone()))
          .expect("release asset not found");

        #[rustfmt::skip]
        let write_webhook_content = |message: &mut String| -> Result<()> {
          writeln!(message, "Server updated to v{version}")?;
          writeln!(message, "<https://github.com/tsukilabs/nil/releases/tag/{tag_name}>")?;
          writeln!(message, "<{asset_url}>")?;
          Ok(())
        };

        let mut message = String::new();
        write_webhook_content(&mut message)?;

        execute_webhook(&webhook_url, &message)?;
      }
    }
  }

  Ok(())
}

fn upload_asset(tag_name: &str, path: &str) -> Result<()> {
  spawn_fmt!("gh release upload --clobber {tag_name} {path} -R tsukilabs/nil")
    .context("failed to upload asset")
}

fn view_release(tag_name: Option<&str>) -> Result<Release> {
  let result = try bikeshed Result<Release> {
    let fields = "assets,body,name,tagName,url";
    let bytes = if let Some(tag_name) = tag_name {
      output_fmt!("gh release view {tag_name} --json {fields} -R tsukilabs/nil")?
    } else {
      output_fmt!("gh release view --json {fields} -R tsukilabs/nil")?
    };

    serde_json::from_slice(&bytes)?
  };

  result.context("failed to fetch github release")
}

fn execute_webhook(url: &str, content: &str) -> Result<()> {
  let result = try bikeshed Result<()> {
    let mut response = ureq::post(url)
      .config()
      .http_status_as_error(false)
      .build()
      .send_json(json!({ "content": content }))?;

    if !response.status().is_success() {
      Err(anyhow!("{}", response.body_mut().read_to_string()?))?;
    }
  };

  result.context("failed to execute webhook")
}

#[derive(Deserialize)]
struct Package {
  version: String,
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Release {
  name: String,
  tag_name: String,
  body: String,
  url: String,
  assets: Vec<Asset>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Asset {
  name: String,
  url: String,
}
