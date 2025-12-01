// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::chat::ChatMessage;
use crate::continent::Coord;
use crate::event::{Event, Listener};
use crate::player::PlayerId;
use crate::world::World;

impl World {
  #[inline]
  pub fn subscribe(&self) -> Listener {
    self.emitter.subscribe()
  }

  /// Emits the event to all players.
  fn broadcast(&self, event: Event) {
    self.emitter.broadcast(event);
  }

  /// Emits the event for a specific player.
  fn emit_to(&self, target: PlayerId, event: Event) {
    self.emitter.emit_to(target, event);
  }

  /// Emits the event to the owner of the city at the specified coordinate, if any.
  fn emit_to_owner(&self, coord: Coord, event: Event) {
    if let Ok(city) = self.city(coord)
      && let Some(player) = city.player()
    {
      self.emitter.emit_to(player, event);
    }
  }

  /// Emits [`Event::ChatUpdated`].
  pub(super) fn emit_chat_updated(&self, message: ChatMessage) {
    self.broadcast(Event::ChatUpdated { message });
  }

  /// Emits [`Event::MilitaryUpdated`].
  pub(super) fn emit_military_updated(&self, player: PlayerId) {
    self.emit_to(player.clone(), Event::MilitaryUpdated { player });
  }

  /// Emits [`Event::PlayerUpdated`].
  pub(super) fn emit_player_updated(&self, player: PlayerId) {
    self.emit_to(player.clone(), Event::PlayerUpdated { player });
  }

  /// Emits [`Event::PublicCityUpdated`].
  pub(super) fn emit_public_city_updated(&self, coord: Coord) {
    self.broadcast(Event::PublicCityUpdated { coord });
  }

  /// Emits [`Event::RoundUpdated`].
  pub(super) fn emit_round_updated(&self) {
    let round = self.round.clone();
    self.broadcast(Event::RoundUpdated { round });
  }

  /// Emits [`Event::CityUpdated`].
  pub(super) fn emit_city_updated(&self, coord: Coord) {
    self.emit_to_owner(coord, Event::CityUpdated { coord });
  }
}
