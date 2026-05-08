#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"
natord = "=1.0.9"

[dependencies.nil-util]
path = "../crates/nil-util"
---

use anyhow::Result;
use natord::compare_ignore_case;
use nil_util::spawn;
use std::fmt::Write as _;
use std::path::Path;
use std::{env, fs};

fn main() -> Result<()> {
  spawn!("cargo test -F typescript export_bindings")?;

  let dir = env::var("TS_RS_EXPORT_DIR")?;
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

  for file in files {
    writeln!(index, "export type {{ {file} }} from './{file}';")?;
  }

  let path = Path::new(&dir).join("index.ts");
  fs::write(path, index)?;

  Ok(())
}
