// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::chat::ChatMessage;
use crate::continent::Coord;
use crate::player::PlayerId;
use crate::round::Round;
use bytes::Bytes;
use nil_util::serde::{from_slice, to_bytes};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::Display;
use tokio::sync::broadcast::{Receiver, Sender, channel};

pub type Listener = Receiver<(Bytes, EventTarget)>;

#[derive(Clone)]
pub(crate) struct Emitter {
  sender: Sender<(Bytes, EventTarget)>,
}

impl Emitter {
  fn new(capacity: usize) -> Self {
    let (sender, _) = channel(capacity);
    Self { sender }
  }

  pub(crate) fn emit(&self, event: Event, target: EventTarget) {
    #[cfg(debug_assertions)]
    tracing::info!(?target, ?event);

    let bytes = Bytes::from(event);
    let _ = self.sender.send((bytes, target));
  }

  pub(crate) fn emit_to(&self, target: PlayerId, event: Event) {
    self.emit(event, EventTarget::Player(target));
  }

  pub(crate) fn broadcast(&self, event: Event) {
    self.emit(event, EventTarget::Broadcast);
  }

  pub(crate) fn subscribe(&self) -> Listener {
    self.sender.subscribe()
  }
}

impl Default for Emitter {
  fn default() -> Self {
    Self::new(100)
  }
}

impl fmt::Debug for Emitter {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Emitter")
      .field("sender", &self.sender.receiver_count())
      .finish()
  }
}

#[derive(Clone, Debug, Display, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[remain::sorted]
pub enum Event {
  /// A new message has been sent in the chat.
  ChatUpdated { message: ChatMessage },

  /// Indicates that there has been a change in the city's data, be it public or not.
  ///
  /// This event is only emitted to the city owner.
  /// If you believe that all players should be notified,
  /// consider using [`Event::PublicCityUpdated`] instead.
  CityUpdated { coord: Coord },

  /// Indicates that the player's military has changed.
  /// It usually means new maneuvers have been initiated,
  /// since the armies themselves are processed at the end of the round.
  MilitaryUpdated { player: PlayerId },

  /// Indicates that there has been a change in some of the player's data, be it public or not.
  PlayerUpdated { player: PlayerId },

  /// Indicates that there has been a change in public data for the city.
  ///
  /// As a rule, whenever the situation requires this event to be emitted,
  /// `Event::CityUpdated` should also be emitted, but the opposite is not true!
  ///
  /// Unlike [`Event::CityUpdated`], which is emitted only to the city owner,
  /// this event is sent to all players in the world.
  PublicCityUpdated { coord: Coord },

  /// Indicates changes in the round, such as the end of a player's turn or
  /// the transition from one round to another, after all players have completed their actions.
  ///
  /// When emitted at the start of the game or at the end of a round,
  /// [`Event::RoundUpdated`] typically makes it unnecessary to emit other events,
  /// as this situation naturally prompts all entities to update themselves.
  RoundUpdated { round: Round },
}

impl From<Event> for Bytes {
  fn from(event: Event) -> Self {
    to_bytes(&event).unwrap()
  }
}

impl From<Bytes> for Event {
  fn from(bytes: Bytes) -> Self {
    from_slice(&bytes).unwrap()
  }
}

#[derive(Clone, Debug)]
pub enum EventTarget {
  Broadcast,
  Player(PlayerId),
}
