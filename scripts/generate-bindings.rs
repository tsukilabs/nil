#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"
natord = "=1.0.9"

[dependencies.clap]
version = "4.6"
features = ["derive"]

[dependencies.nil-util]
path = "../crates/nil-util"
---

use anyhow::Result;
use clap::Parser;
use natord::compare_ignore_case;
use nil_util::spawn;
use std::fmt::Write as _;
use std::path::Path;
use std::{env, fs};

#[derive(Parser)]
struct Args {
  #[arg(long)]
  force: bool,
}

fn main() -> Result<()> {
  let args = Args::parse();
  let dir = env::var("TS_RS_EXPORT_DIR")?;

  if args.force {
    fs::remove_dir_all(&dir)?;
  }

  spawn!("cargo test -F typescript export_bindings")?;

  let mut files = Vec::new();

  for entry in fs::read_dir(&dir)? {
    let path = entry?.path();
    if path.is_file()
      && let Some(extension) = path.extension()
      && extension.eq_ignore_ascii_case("ts")
      && let Some(stem) = path.file_stem()
      && let Some(stem) = stem.to_str()
      && !stem.eq_ignore_ascii_case("index")
    {
      files.push(stem.to_owned());
    }
  }

  files.sort_by(|a, b| compare_ignore_case(a, b));

  let mut index = String::new();
  write_header(&mut index)?;

  for file in files {
    writeln!(index, "export type {{ {file} }} from './{file}';")?;
  }

  let path = Path::new(&dir).join("index.ts");
  fs::write(path, index)?;

  spawn!("pnpm run -F @tsukilabs/nil-bindings build")?;

  Ok(())
}

#[rustfmt::skip]
fn write_header(index: &mut String) -> Result<()> {
  writeln!(index, "//! Copyright (C) Call of Nil contributors")?;
  writeln!(index, "//! SPDX-License-Identifier: AGPL-3.0-only\n")?;
  writeln!(index, "import {{ version }} from '../package.json' with {{ type: 'json' }};\n")?;
  writeln!(index, "export const VERSION = version;\n")?;

  Ok(())
}
