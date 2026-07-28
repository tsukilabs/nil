---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"
csbindgen = "=1.9.6"
natord = "=1.0.9"
regex = "1.13"

[dependencies.clap]
version = "4.6"
features = ["derive"]

[dependencies.nil-ffi-node]
path = "../crates/nil-ffi-node"

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

  #[arg(long)]
  skip_ffi_cs: bool,

  #[arg(long)]
  skip_ffi_node: bool,

  #[arg(long)]
  skip_ts: bool,
}

fn main() -> Result<()> {
  let args = Args::parse();

  if !args.skip_ts {
    generate_ts(args.force)?;
  }

  if !args.skip_ffi_node {
    generate_ffi_node()?;
  }

  if !args.skip_ffi_cs {
    generate_ffi_cs()?;
  }

  Ok(())
}

fn generate_ts(force: bool) -> Result<()> {
  let dir = env::var("TS_RS_EXPORT_DIR")?;

  if force && fs::exists(&dir)? {
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

fn generate_ffi_node() -> Result<()> {
  nil_ffi_node::generate()
    .input("crates/nil-ffi/src/lib.rs")
    .output("packages/ffi/src/def.ts")
    .call()?;

  Ok(())
}

fn generate_ffi_cs() -> Result<()> {
  csbindgen::Builder::default()
    .input_extern_file("crates/nil-ffi/src/lib.rs")
    .input_extern_file("crates/nil-ffi/src/status.rs")
    .csharp_dll_name("libnil")
    .csharp_class_name("libnil")
    .csharp_class_accessibility("public")
    .csharp_type_rename(rename_cs_type)
    .generate_csharp_file("crates/nil-ffi/gen/libnil.cs")
    .unwrap();

  Ok(())
}

fn rename_cs_type(name: String) -> String {
  match name.as_str() {
    "RequestId" => "uint".into(),
    "Status" => "StatusCode".into(),
    _ => name,
  }
}
