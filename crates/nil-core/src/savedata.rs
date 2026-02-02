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
use std::fmt;
use std::io::{Read, Write};
use tar::Archive;

const MINIFY: bool = cfg!(any(
  not(debug_assertions),
  target_os = "android",
  target_os = "ios"
));

type TarBuilder<'a> = tar::Builder<GzEncoder<&'a mut Vec<u8>>>;

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
  pub fn read(bytes: &[u8]) -> Result<Self> {
    read_tar(bytes, "world")
      .inspect_err(|err| tracing::error!(message = %err, error = ?err))
      .map_err(|_| Error::FailedToReadSavedata)
  }

  pub(crate) fn write(&mut self, buffer: &mut Vec<u8>) -> Result<()> {
    for player in self.player_manager.players_mut() {
      *player.status_mut() = PlayerStatus::Inactive;
    }

    write_tar(buffer, self)
      .inspect_err(|err| tracing::error!(message = %err, error = ?err))
      .map_err(|_| Error::FailedToWriteSavedata)
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

  pub fn read(bytes: &[u8]) -> Result<Self> {
    read_tar(bytes, "info")
      .inspect_err(|err| tracing::error!(message = %err, error = ?err))
      .map_err(|_| Error::FailedToReadSavedata)
  }
}

fn read_tar<T>(bytes: &[u8], entry_name: &str) -> AnyResult<T>
where
  T: DeserializeOwned,
{
  let decoder = GzDecoder::new(bytes);
  let mut archive = Archive::new(decoder);

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

fn write_tar(buffer: &mut Vec<u8>, data: &Savedata) -> AnyResult<()> {
  let encoder = GzEncoder::new(buffer, Compression::best());
  let mut tar_builder = TarBuilder::new(encoder);

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

pub struct SaveHandle(Box<dyn FnOnce(Vec<u8>) + Send + Sync>);

impl SaveHandle {
  pub fn new<F>(f: F) -> Self
  where
    F: FnOnce(Vec<u8>) + Send + Sync + 'static,
  {
    Self(Box::new(f))
  }

  #[inline]
  pub fn save(self, data: Vec<u8>) {
    (self.0)(data);
  }
}

impl fmt::Debug for SaveHandle {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("SaveHandle")
      .finish_non_exhaustive()
  }
}
