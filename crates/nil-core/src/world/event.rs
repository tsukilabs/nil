// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::chat::ChatMessage;
use crate::continent::coord::Coord;
use crate::error::Result;
use crate::event::{Event, Listener};
use crate::military::maneuver::ManeuverId;
use crate::player::PlayerId;
use crate::report::ReportKind;
use crate::report::battle::BattleReport;
use crate::report::support::SupportReport;
use crate::world::World;

impl World {
  #[inline]
  pub fn subscribe(&self) -> Listener {
    self.emitter.subscribe()
  }

  /// Emits the event to all players.
  fn broadcast(&self, event: Event) -> Result<()> {
    self.emitter.broadcast(event)
  }

  /// Emits the event for a specific player.
  fn emit_to(&self, target: PlayerId, event: Event) -> Result<()> {
    self.emitter.emit_to(target, event)
  }

  /// Emits the event to the owner of the city at the specified coordinate, if any.
  fn emit_to_city_owner(&self, coord: Coord, event: Event) -> Result<()> {
    let city = self.city(coord)?;
    if let Some(player) = city.player() {
      self.emitter.emit_to(player, event)?;
    }

    Ok(())
  }

  /// Emits [`Event::ChatMessage`].
  pub(super) fn emit_chat_message(&self, message: ChatMessage) -> Result<()> {
    let world = self.config.id();
    self.broadcast(Event::ChatMessage { world, message })
  }

  /// Emits [`Event::City`].
  pub(super) fn emit_city(&self, coord: Coord) -> Result<()> {
    let world = self.config.id();
    self.emit_to_city_owner(coord, Event::City { world, coord })
  }

  /// Emits [`Event::Drop`].
  /// This should never be called manually.
  pub(super) fn emit_drop(&self) -> Result<()> {
    let world = self.config.id();
    self.broadcast(Event::Drop { world })
  }

  /// Emits [`Event::Military`].
  pub(super) fn emit_military(&self, player: PlayerId) -> Result<()> {
    let world = self.config.id();
    self.emit_to(player.clone(), Event::Military { world, player })
  }

  /// Emits [`Event::Military`] to all players participating in the specified maneuver.
  pub(super) fn emit_military_to_maneuver_players(&self, id: ManeuverId) -> Result<()> {
    let maneuver = self.military.maneuver(id)?;
    let sender = self.city(maneuver.origin())?.player();
    let target = self.city(maneuver.destination())?.player();

    if let Some(sender) = &sender {
      self.emit_military(sender.clone())?;
    }

    if let Some(target) = target
      && sender.is_none_or(|it| it != target)
    {
      self.emit_military(target)?;
    }

    Ok(())
  }

  /// Emits [`Event::Player`].
  pub(super) fn emit_player(&self, player: PlayerId) -> Result<()> {
    let world = self.config.id();
    self.emit_to(player.clone(), Event::Player { world, player })
  }

  /// Emits [`Event::PublicCity`].
  pub(super) fn emit_public_city(&self, coord: Coord) -> Result<()> {
    let world = self.config.id();
    self.broadcast(Event::PublicCity { world, coord })
  }

  /// Emits [`Event::Report`].
  pub(super) fn emit_report(&self, player: PlayerId, report: ReportKind) -> Result<()> {
    let world = self.config.id();
    self.emit_to(player, Event::Report { world, report: Box::new(report) })
  }

  pub(super) fn emit_battle_report(&self, report: BattleReport) -> Result<()> {
    if let Some(attacker) = report.attacker().player().cloned() {
      self.emit_report(attacker, report.clone().into())?;
    }

    if let Some(defender) = report.defender().player().cloned() {
      debug_assert_ne!(report.attacker().player(), Some(&defender));
      self.emit_report(defender, report.into())?;
    }

    Ok(())
  }

  pub(super) fn emit_support_report(&self, report: SupportReport) -> Result<()> {
    if let Some(sender) = report.sender().player().cloned() {
      self.emit_report(sender, report.clone().into())?;
    }

    if let Some(receiver) = report.receiver().player().cloned() {
      debug_assert_ne!(report.sender().player(), Some(&receiver));
      self.emit_report(receiver, report.into())?;
    }

    Ok(())
  }

  /// Emits [`Event::Round`].
  pub(super) fn emit_round(&self) -> Result<()> {
    let world = self.config.id();
    let round = self.round.clone();
    self.broadcast(Event::Round { world, round })
  }
}
