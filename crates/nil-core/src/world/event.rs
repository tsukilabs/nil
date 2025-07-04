// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::chat::ChatMessage;
use crate::event::{Event, Listener};
use crate::infrastructure::building::prefecture::{
  PrefectureBuildOrderId,
  PrefectureBuildOrderKind,
};
use crate::player::{Player, PlayerId, PlayerStatus};
use crate::village::{Coord, VillagePublicState};

#[cfg(debug_assertions)]
use tracing::info;

impl World {
  #[inline]
  pub fn subscribe(&self) -> Listener {
    self.emitter.subscribe()
  }

  pub(super) fn emit_to(&self, target: PlayerId, event: Event) {
    #[cfg(debug_assertions)]
    info!(%target, ?event);

    self.emitter.emit_to(target, event);
  }

  pub(super) fn broadcast(&self, event: Event) {
    #[cfg(debug_assertions)]
    info!(?event);

    self.emitter.broadcast(event);
  }

  pub(super) fn emit_chat_message(&self, message: ChatMessage) {
    self.broadcast(Event::ChatMessage { message });
  }

  pub(super) fn emit_guest_left(&self, guest: Player) {
    self.broadcast(Event::GuestLeft { guest });
  }

  pub(super) fn emit_player_resources_updated(&self, target: PlayerId) {
    self.emit_to(target, Event::PlayerResourcesUpdated);
  }

  pub(super) fn emit_player_spawned(&self, player: Player) {
    self.broadcast(Event::PlayerSpawned { player });
  }

  pub(super) fn emit_player_status_updated(&self, id: PlayerId, status: PlayerStatus) {
    self.broadcast(Event::PlayerStatusUpdated { player: id, status });
  }

  pub(super) fn emit_prefecture_build_queue_updated(
    &self,
    target: PlayerId,
    coord: Coord,
    id: PrefectureBuildOrderId,
    order_kind: PrefectureBuildOrderKind,
  ) {
    let event = Event::PrefectureBuildQueueUpdated { coord, id, order_kind };
    self.emit_to(target, event);
  }

  pub(super) fn emit_round_update(&self) {
    let round = self.round.clone();
    self.broadcast(Event::RoundUpdated { round });
  }

  pub(super) fn emit_village_spawned(&self, village: VillagePublicState) {
    self.broadcast(Event::VillageSpawned { village });
  }
}
