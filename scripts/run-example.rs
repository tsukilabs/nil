#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"

[dependencies.clap]
version = "4.5"
features = ["derive"]

[dependencies.nil-util]
path = "../crates/nil-util"
---

use anyhow::Result;
use clap::Parser;
use nil_util::spawn;
use std::fmt::Write;

#[derive(Parser)]
struct Args {
  script: String,

  #[arg(short = 'r', long)]
  release: bool,

  #[arg(long)]
  server: Option<String>,

  #[arg(long)]
  player: Option<String>,

  #[arg(long)]
  player_password: Option<String>,

  #[arg(long)]
  world: Option<String>,

  #[arg(long)]
  world_password: Option<String>,

  #[arg(long)]
  token: Option<String>,
}

fn main() -> Result<()> {
  let args = Args::parse();
  let mut command = if args.release {
    String::from("nls ")
  } else {
    String::from("cargo run -p nil-lua-cli -- ")
  };

  let script = args.script.trim_end_matches(".lua");
  write!(command, " crates/nil-lua/examples/{script}.lua")?;

  macro_rules! set {
    ($flag:literal, $value:ident) => {
      if let Some(value) = args.$value {
        let flag = $flag;
        write!(command, " --{flag} {value}")?;
      }
    };
  }

  set!("server", server);
  set!("player", player);
  set!("player-password", player_password);
  set!("world", world);
  set!("world-password", world_password);
  set!("token", token);

  spawn!(command)
}
