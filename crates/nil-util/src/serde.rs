// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use flate2::Compression;
use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::to_vec;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

pub use bytes::Bytes;
pub use serde_json::from_slice;

pub fn to_bytes<T>(value: &T) -> Result<Bytes>
where
  T: ?Sized + Serialize,
{
  Ok(to_vec(value).map(Bytes::from)?)
}

#[bon::builder]
pub fn read_file<T>(
  #[builder(start_fn)] path: impl AsRef<Path>,
  #[builder(default)] decode: bool,
) -> Result<T>
where
  T: DeserializeOwned,
{
  let mut file = File::open_buffered(path)?;
  let mut buf = Vec::new();

  if decode {
    let mut gz = GzDecoder::new(file);
    gz.read_to_end(&mut buf)?;
  } else {
    file.read_to_end(&mut buf)?;
  }

  Ok(from_slice(&buf)?)
}

#[bon::builder]
pub fn write_file<T>(
  #[builder(start_fn)] path: impl AsRef<Path>,
  #[builder(start_fn)] value: &T,
  #[builder(default)] encode: bool,
) -> Result<()>
where
  T: ?Sized + Serialize,
{
  let path = path.as_ref();
  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent)?;
  }

  let mut file = File::create(path)?;
  let bytes = to_vec(value)?;

  if encode {
    let mut e = GzEncoder::new(file, Compression::best());
    e.write_all(&bytes)?;
    e.finish()?.flush()?;
  } else {
    file.write_all(&bytes)?;
    file.flush()?;
  }

  Ok(())
}
