#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0"
itertools = "0.14"
natord = "=1.0.9"

[dependencies.nil-util]
path = "../crates/nil-util"
---

#![feature(file_buffered)]

use anyhow::Result;
use itertools::Itertools;
use natord::compare_ignore_case;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::BufRead;

fn main() -> Result<()> {
  let dir = "crates/nil-namegen/data";
  for entry in fs::read_dir(dir)? {
    let entry = entry?;
    let path = entry.path();
    if entry.file_type()?.is_file()
      && let Some(extension) = path.extension()
      && let Some(extension) = extension.to_str()
      && extension == "csv"
    {
      let mut names = HashSet::new();
      let file = File::open_buffered(&path)?;

      for line in file.lines() {
        names.insert(line?.trim().to_owned());
      }

      let contents = names
        .into_iter()
        .sorted_by(|a, b| compare_ignore_case(a, b))
        .join("\n")
        .trim()
        .to_owned();

      fs::write(path, contents)?;
    }
  }

  Ok(())
}
