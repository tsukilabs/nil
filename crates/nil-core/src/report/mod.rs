// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod battle;
pub mod support;

use crate::error::{Error, Result};
use crate::player::PlayerId;
use crate::round::RoundId;
use battle::BattleReport;
use itertools::Itertools;
use jiff::Zoned;
use nil_util::iter::IterExt;
use nil_util::vec::VecExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use support::SupportReport;
use uuid::Uuid;

pub trait Report {
  fn id(&self) -> ReportId;
  fn round(&self) -> RoundId;
  fn time(&self) -> &Zoned;
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReportManager {
  reports: HashMap<ReportId, ReportKind>,
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

  /// Forwards a report to a player.
  pub(crate) fn forward(&mut self, id: ReportId, recipient: PlayerId) -> bool {
    self.reports.contains_key(&id)
      && self
        .players
        .entry(recipient)
        .or_default()
        .push_unique(id)
        .is_none()
  }

  /// Removes a report from a player's list of reports.
  pub(crate) fn remove_of(&mut self, id: ReportId, player: &PlayerId) {
    if let Some(reports) = self.players.get_mut(player) {
      reports.retain(|report| *report != id);
    }
  }

  /// Removes reports that are not associated with any player.
  pub(crate) fn prune(&mut self) {
    let reports = self
      .players
      .values()
      .flat_map(|ids| ids.iter().copied())
      .collect_set();

    self
      .reports
      .retain(|id, _| reports.contains(id));
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
#[remain::sorted]
pub enum ReportKind {
  Battle { report: Arc<BattleReport> },
  Support { report: Arc<SupportReport> },
}

impl ReportKind {
  pub fn as_dyn(&self) -> &dyn Report {
    match self {
      Self::Battle { report } => report.as_ref(),
      Self::Support { report } => report.as_ref(),
    }
  }

  #[inline]
  pub fn id(&self) -> ReportId {
    self.as_dyn().id()
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
