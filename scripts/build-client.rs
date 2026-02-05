#!/usr/bin/env cargo
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

[dependencies.open]
version = "5.3"
features = ["shellexecute-on-windows"]

[dependencies.serde]
version = "1.0"
features = ["derive"]
---

use anyhow::Result;
use clap::Parser;
use nil_util::{spawn, spawn_fmt};
use serde::Deserialize;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
  #[arg(long)]
  android: bool,
  #[arg(long)]
  kanata: bool,
  #[arg(long)]
  open_preview: bool,
  #[arg(long)]
  preview: bool,
  #[arg(long)]
  target_dir: Option<PathBuf>,
  #[arg(long)]
  wasm: bool,
}

fn main() -> Result<()> {
  let args = Args::parse();
  if args.wasm {
    spawn!("pnpm run wasm --release")?;
  }

  let mut command = if args.android {
    String::from("cargo tauri android build --apk")
  } else {
    String::from("cargo tauri build")
  };

  let mut env = Vec::new();

  if args.preview {
    env.push(("NIL_MINIFY_SOURCE", "false"));
    write!(command, " --no-bundle -- --profile preview")?;
  } else {
    write!(command, " -- --release")?;
  }

  spawn!(command, env)?;

  if args.android {
    let path =
      "app/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk";

    let package = fs::read("package.json")?;
    let package = serde_json::from_slice::<Package>(&package)?;
    let name = format!("Call.of.Nil_{}.apk", package.version);

    if args.kanata {
      spawn_fmt!("kanata add --path {path} --name {name}")?;
    }

    if let Some(target_dir) = args.target_dir {
      fs::copy(path, target_dir.join(name))?;
    }
  }

  if args.preview && args.open_preview {
    if cfg!(windows) {
      open::that_detached("target/preview/nil.exe")?;
    } else {
      open::that_detached("target/preview/nil")?;
    }
  }

  Ok(())
}

#[derive(Deserialize)]
struct Package {
  version: String,
}
