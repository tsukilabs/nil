// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod battle;

use crate::error::{Error, Result};
use crate::player::PlayerId;
use itertools::Itertools;
use jiff::Zoned;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use uuid::Uuid;

pub use battle::BattleReport;

pub trait Report {
  fn id(&self) -> ReportId;
  fn timestamp(&self) -> &Zoned;
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReportManager {
  reports: BTreeMap<ReportId, ReportKind>,
  players: HashMap<PlayerId, Vec<ReportId>>,
}

impl ReportManager {
  pub(crate) fn manage<I>(&mut self, report: ReportKind, players: I)
  where
    I: IntoIterator<Item = PlayerId>,
  {
    let id = report.as_dyn().id();
    let players = players.into_iter().unique().collect_vec();

    if !players.is_empty() {
      for player in players {
        self
          .players
          .entry(player)
          .or_default()
          .push(id);
      }

      debug_assert!(!self.reports.contains_key(&id));
      self.reports.insert(id, report);
    }
  }

  pub fn report(&self, id: ReportId) -> Result<&ReportKind> {
    self
      .reports
      .get(&id)
      .ok_or(Error::ReportNotFound(id))
  }

  pub fn reports_by<F>(&self, f: F) -> impl Iterator<Item = &ReportKind>
  where
    F: Fn((ReportId, &ReportKind)) -> bool,
  {
    self
      .reports
      .iter()
      .filter(move |(id, report)| f((**id, report)))
      .map(|(_, report)| report)
  }

  pub fn reports_of(&self, player: &PlayerId) -> impl Iterator<Item = ReportId> {
    self
      .players
      .get(player)
      .map(Vec::as_slice)
      .unwrap_or_default()
      .iter()
      .copied()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
#[remain::sorted]
pub enum ReportKind {
  Battle { report: Arc<BattleReport> },
}

impl ReportKind {
  pub fn as_dyn(&self) -> &dyn Report {
    match self {
      Self::Battle { report } => report.as_ref(),
    }
  }
}

#[must_use]
#[derive(
  Clone,
  Copy,
  Debug,
  derive_more::Display,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  Deserialize,
  Serialize,
)]
pub struct ReportId(Uuid);

impl ReportId {
  #[inline]
  pub fn new() -> Self {
    Self(Uuid::now_v7())
  }
}

impl Default for ReportId {
  fn default() -> Self {
    Self::new()
  }
}
