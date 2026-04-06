#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"
regex = "1.12"
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

use anyhow::{Result, bail};
use clap::Parser;
use nil_util::{output_fmt, spawn, spawn_fmt};
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use std::borrow::Cow;
use std::fmt::Write;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
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
      update_server(&token)?;

      let release = view_release(Some(&tag_name))?;

      if let Ok(webhook_url) = env::var("NIL_DISCORD_WEBHOOK_URL_PRIVATE") {
        let asset_url = release
          .assets
          .iter()
          .find_map(|asset| (asset.name == asset_name).then(|| asset.url.clone()))
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

        execute_webhook(&webhook_url, &message)?;
      }

      if let Ok(webhook_url) = env::var("NIL_DISCORD_WEBHOOK_URL_RELEASE") {
        execute_release_webhook(&webhook_url, &release)?;
      }
    }
  }

  Ok(())
}

fn update_server(token: &str) -> Result<()> {
  let mut tries = 1;
  loop {
    let status = ureq::get("https://tsukilabs.dev.br/release/nil")
      .header("Authorization", format!("Bearer {token}"))
      .config()
      .http_status_as_error(false)
      .build()
      .call()?
      .status();

    if status.is_success() {
      return Ok(());
    } else if status.is_server_error() && tries < 4 {
      tries += 1;
      sleep(Duration::from_secs(5 * tries));
      continue;
    }

    bail!("Failed to update server: {status}");
  }
}

fn view_release(tag_name: Option<&str>) -> Result<Release> {
  let fields = "assets,body,name,tagName,url";
  let bytes = if let Some(tag_name) = tag_name {
    output_fmt!("gh release view {tag_name} --json {fields} -R tsukilabs/nil")?
  } else {
    output_fmt!("gh release view --json {fields} -R tsukilabs/nil")?
  };

  Ok(serde_json::from_slice(&bytes)?)
}

fn upload_asset(tag_name: &str, path: &str) -> Result<()> {
  spawn_fmt!("gh release upload --clobber {tag_name} {path} -R tsukilabs/nil")?;
  Ok(())
}

fn execute_webhook(url: &str, content: &str) -> Result<()> {
  ureq::post(url).send_json(json!({ "content": content }))?;
  Ok(())
}

fn execute_release_webhook(url: &str, release: &Release) -> Result<()> {
  let mut message = format!("# {}\n\n", release.name);

  let pr_re = Regex::new(r"\*\s+(.+?by\s+)@(\S+)\s+in\s+https.+?pull/(\d+)")?;
  let changelog_re = Regex::new(r".+?Full\sChangelog.+?(https\S+)")?;

  for line in release.body.trim().split_inclusive('\n') {
    let mut current_line = Cow::Borrowed(line);

    if let Some(captures) = pr_re.captures(line)
      && let Some(title) = captures.get(1).map(|it| it.as_str())
      && let Some(user) = captures.get(2).map(|it| it.as_str())
      && let Some(pr) = captures.get(3).map(|it| it.as_str())
    {
      let profile = format!("<https://github.com/{user}>");
      let pr_url = format!("<https://github.com/tsukilabs/nil/pull/{pr}>");
      let line = format!("- {title}[@{user}]({profile}) in [#{pr}]({pr_url})\n");
      current_line = Cow::Owned(line);
    }

    if let Some(captures) = changelog_re.captures(line)
      && let Some(url) = captures.get(1).map(|it| it.as_str())
    {
      let line = format!("**Full Changelog**: <{url}>\n");
      current_line = Cow::Owned(line);
    };

    message.push_str(&current_line);
  }

  message = message.replace("\n\n\n", "\n\n");

  execute_webhook(url, message.trim())?;

  Ok(())
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
