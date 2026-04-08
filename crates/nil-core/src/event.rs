// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::chat::ChatMessage;
use crate::continent::Coord;
use crate::error::{Error, Result};
use crate::player::PlayerId;
use crate::report::ReportId;
use crate::round::Round;
use crate::world::config::WorldId;
use bytes::Bytes;
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
  pub fn new(capacity: usize) -> Self {
    let (sender, _) = channel(capacity);
    Self { sender }
  }

  pub(crate) fn emit(&self, event: Event, target: EventTarget) -> Result<()> {
    tracing::info!(?target, ?event);
    let bytes = Bytes::try_from(event)?;
    let _ = self.sender.send((bytes, target));
    Ok(())
  }

  pub(crate) fn emit_to(&self, target: PlayerId, event: Event) -> Result<()> {
    self.emit(event, EventTarget::Player(target))
  }

  pub(crate) fn broadcast(&self, event: Event) -> Result<()> {
    self.emit(event, EventTarget::Broadcast)
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
  ChatUpdated {
    world: WorldId,
    message: ChatMessage,
  },

  /// Indicates that there has been a change in the city's data, be it public or not.
  ///
  /// This event is only emitted to the city owner.
  /// If you believe that all players should be notified,
  /// consider using [`Event::PublicCityUpdated`] instead.
  CityUpdated { world: WorldId, coord: Coord },

  /// Signals that all active players should disconnect, as the world is about to be dropped.
  ///
  /// This event **MUST** only be emitted inside the `Drop` implementation of the `World` struct.
  Drop { world: WorldId },

  /// Indicates that the player's military has changed.
  /// It usually means new maneuvers have been initiated,
  /// since the armies themselves are processed at the end of the round.
  MilitaryUpdated { world: WorldId, player: PlayerId },

  /// Indicates that there has been a change in some of the player's data, be it public or not.
  PlayerUpdated { world: WorldId, player: PlayerId },

  /// Indicates that there has been a change in public data for the city.
  ///
  /// As a rule, whenever the situation requires this event to be emitted,
  /// `Event::CityUpdated` should also be emitted, but the opposite is not true!
  ///
  /// Unlike [`Event::CityUpdated`], which is emitted only to the city owner,
  /// this event is sent to all players in the world.
  PublicCityUpdated { world: WorldId, coord: Coord },

  /// A report was generated.
  Report { world: WorldId, report: ReportId },

  /// Indicates changes in the round, such as the end of a player's turn or
  /// the transition from one round to another, after all players have completed their actions.
  ///
  /// When emitted at the start of the game or at the end of a round,
  /// [`Event::RoundUpdated`] typically makes it unnecessary to emit other events,
  /// as this situation naturally prompts all entities to update themselves.
  RoundUpdated { world: WorldId, round: Round },
}

impl TryFrom<Event> for Bytes {
  type Error = Error;

  fn try_from(event: Event) -> Result<Self> {
    serde_json::to_vec(&event)
      .map(Bytes::from)
      .map_err(|err| {
        tracing::error!("Failed to serialize event: {err}");
        Error::FailedToSerializeEvent
      })
  }
}

impl TryFrom<Bytes> for Event {
  type Error = Error;

  fn try_from(bytes: Bytes) -> Result<Self> {
    serde_json::from_slice(&bytes).map_err(|err| {
      tracing::error!("Failed to deserialize event: {err}");
      Error::FailedToDeserializeEvent
    })
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventTarget {
  Broadcast,
  Player(PlayerId),
}
