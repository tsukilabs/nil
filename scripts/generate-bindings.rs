---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"
natord = "=1.0.9"
regex = "1.13"

[dependencies.clap]
version = "4.6"
features = ["derive"]

[dependencies.nil-util]
path = "../crates/nil-util"
---

#![feature(file_buffered)]

use anyhow::{Result, bail};
use clap::Parser;
use natord::compare_ignore_case as compare;
use nil_util::spawn;
use regex::regex;
use std::fmt::Write as _;
use std::fs::File;
use std::io::BufRead;
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

  if args.force && fs::exists(&dir)? {
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
      files.push(Binding {
        name: stem.to_owned(),
        kind: BindingKind::from_file(&path)?,
      });
    }
  }

  files.sort_by(|a, b| compare(&a.name, &b.name));

  let mut index = String::new();
  write_license(&mut index)?;
  write_index_exports(&mut index, &files)?;

  let path = Path::new(&dir).join("index.ts");
  fs::write(path, index)?;

  spawn!("pnpm run -F @tsukilabs/nil-bindings build")?;

  Ok(())
}

fn write_license(index: &mut String) -> Result<()> {
  writeln!(index, "//! Copyright (C) Call of Nil contributors")?;
  writeln!(index, "//! SPDX-License-Identifier: AGPL-3.0-only\n")?;
  Ok(())
}

#[rustfmt::skip]
fn write_index_exports(index: &mut String, bindings: &[Binding]) -> Result<()> {
  writeln!(index, "import {{ version }} from '../package.json' with {{ type: 'json' }};\n")?;
  writeln!(index, "export const VERSION = version;\n")?;

  for binding in bindings {
    let name = &binding.name;
    if let BindingKind::Type = binding.kind {
      writeln!(index, "export type {{ {name} }} from './{name}';")?;
    } else {
      writeln!(index, "export {{ {name} }} from './{name}';")?;
    }
  }

  Ok(())
}

struct Binding {
  name: String,
  kind: BindingKind,
}

enum BindingKind {
  Enum,
  Type,
}

impl BindingKind {
  fn from_file(path: &Path) -> Result<Self> {
    let file = File::open_buffered(path)?;
    let regex = regex!(r"export\s+(\w+)\s");
    for line in file.lines() {
      let line = line?;
      if !line.starts_with("//")
        && let Some(captures) = regex.captures(&line)
        && let Some(keyword) = captures.get(1)
      {
        return match keyword.as_str() {
          "enum" => Ok(Self::Enum),
          _ => Ok(Self::Type),
        };
      }
    }

    bail!("unknown binding kind at {}", path.to_string_lossy());
  }
}
