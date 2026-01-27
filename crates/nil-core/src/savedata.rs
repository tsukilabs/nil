// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::chat::Chat;
use crate::continent::Continent;
use crate::error::{AnyResult, Error, Result};
use crate::military::Military;
use crate::npc::bot::BotManager;
use crate::npc::precursor::PrecursorManager;
use crate::player::{PlayerManager, PlayerStatus};
use crate::ranking::Ranking;
use crate::report::ReportManager;
use crate::round::{Round, RoundId};
use crate::world::{WorldConfig, WorldName, WorldStats};
use anyhow::bail;
use flate2::Compression;
use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use jiff::Zoned;
use semver::Version;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use tar::Archive;

const MINIFY: bool = cfg!(any(
  not(debug_assertions),
  target_os = "android",
  target_os = "ios"
));

type TarBuilder = tar::Builder<GzEncoder<File>>;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Savedata {
  pub(crate) round: Round,
  pub(crate) continent: Continent,
  pub(crate) player_manager: PlayerManager,
  pub(crate) bot_manager: BotManager,
  pub(crate) precursor_manager: PrecursorManager,
  pub(crate) military: Military,
  pub(crate) ranking: Ranking,
  pub(crate) report: ReportManager,
  pub(crate) config: WorldConfig,
  pub(crate) stats: WorldStats,
  pub(crate) chat: Chat,
}

impl Savedata {
  pub fn read(path: &Path) -> Result<Self> {
    read_tar(path, "world").map_err(|_| Error::FailedToReadSavedata)
  }

  pub(crate) fn write(&mut self, path: &Path) -> Result<()> {
    for player in self.player_manager.players_mut() {
      *player.status_mut() = PlayerStatus::Inactive;
    }

    write_tar(path, self).map_err(|_| Error::FailedToWriteSavedata)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedataInfo {
  world_name: WorldName,
  round: RoundId,
  version: Version,
  saved_at: Zoned,
}

impl SavedataInfo {
  fn new(data: &Savedata) -> AnyResult<Self> {
    Ok(Self {
      world_name: data.config.name(),
      round: data.round.id(),
      version: Version::parse(env!("CARGO_PKG_VERSION"))?,
      saved_at: Zoned::now(),
    })
  }

  pub fn read(path: &Path) -> Result<Self> {
    read_tar(path, "info").map_err(|_| Error::FailedToReadSavedata)
  }
}

fn read_tar<T>(path: &Path, entry_name: &str) -> AnyResult<T>
where
  T: DeserializeOwned,
{
  let file = File::open_buffered(path)?;
  let file = GzDecoder::new(file);
  let mut archive = Archive::new(file);

  for entry in archive.entries()? {
    let mut entry = entry?;
    if let Ok(entry_path) = entry.path()
      && let Some(path) = entry_path.to_str()
      && path == entry_name
    {
      let size = entry.size().try_into()?;
      let mut buffer = Vec::with_capacity(size);
      entry.read_to_end(&mut buffer)?;
      return Ok(serde_json::from_slice(&buffer)?);
    }
  }

  bail!("Entry not found: {entry_name}");
}

fn write_tar(path: &Path, data: &Savedata) -> AnyResult<()> {
  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent)?;
  }

  let file = File::create(path)?;
  let file = GzEncoder::new(file, Compression::best());
  let mut tar_builder = TarBuilder::new(file);

  let info = SavedataInfo::new(data)?;
  append(&mut tar_builder, &info, "info")?;
  append(&mut tar_builder, data, "world")?;

  tar_builder.into_inner()?.finish()?.flush()?;

  Ok(())
}

fn append<T>(builder: &mut TarBuilder, value: &T, path: &str) -> AnyResult<()>
where
  T: ?Sized + Serialize,
{
  let bytes = if MINIFY {
    serde_json::to_vec(value)?
  } else {
    serde_json::to_vec_pretty(value)?
  };

  let mut header = tar::Header::new_gnu();
  header.set_size(bytes.len().try_into()?);
  header.set_cksum();

  builder.append_data(&mut header, path, bytes.as_slice())?;

  Ok(())
}
