// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::chat::ChatMessage;
use crate::continent::coord::Coord;
use crate::error::{Error, Result};
use crate::player::PlayerId;
use crate::report::ReportKind;
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
    tracing::trace!(?target, ?event);
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

#[derive(Clone, Display, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[remain::sorted]
pub enum Event {
  /// A new message has been sent in the chat.
  ChatMessage {
    world: WorldId,
    message: ChatMessage,
  },

  /// Indicates that there has been a change in the city's data, be it public or not.
  ///
  /// This event is only emitted to the city owner.
  /// If you believe that all players should be notified,
  /// consider using [`Event::PublicCity`] instead.
  City { world: WorldId, coord: Coord },

  /// Signals that all active players should disconnect, as the world is about to be dropped.
  ///
  /// This event **MUST** only be emitted inside the `Drop` implementation of the `World` struct.
  Drop { world: WorldId },

  /// Indicates that the player's military has changed.
  /// It usually means new maneuvers have been initiated,
  /// since the armies themselves are processed at the end of the round.
  Military { world: WorldId, player: PlayerId },

  /// Indicates that there has been a change in some of the player's data, be it public or not.
  Player { world: WorldId, player: PlayerId },

  /// Indicates that there has been a change in public data for the city.
  ///
  /// As a rule, whenever the situation requires this event to be emitted,
  /// [`Event::City`] should also be emitted, but the opposite is not true!
  ///
  /// Unlike [`Event::City`], which is emitted only to the city owner,
  /// this event is sent to all players in the world.
  PublicCity { world: WorldId, coord: Coord },

  /// A report was generated.
  Report {
    world: WorldId,
    report: Box<ReportKind>,
  },

  /// Indicates changes in the round, such as the end of a player's turn or
  /// the transition from one round to another, after all players have completed their actions.
  ///
  /// When emitted at the start of the game or at the end of a round,
  /// [`Event::Round`] typically makes it unnecessary to emit other events,
  /// as this situation naturally prompts all entities to update themselves.
  Round { world: WorldId, round: Round },
}

impl fmt::Debug for Event {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ChatMessage { world, message } => {
        f.debug_struct("ChatMessage")
          .field("world", world)
          .field("message", &message.id())
          .finish()
      }
      Self::City { world, coord } => {
        f.debug_struct("City")
          .field("world", world)
          .field("coord", coord)
          .finish()
      }
      Self::Drop { world } => {
        f.debug_struct("Drop")
          .field("world", world)
          .finish()
      }
      Self::Military { world, player } => {
        f.debug_struct("Military")
          .field("world", world)
          .field("player", player)
          .finish()
      }
      Self::Player { world, player } => {
        f.debug_struct("Player")
          .field("world", world)
          .field("player", player)
          .finish()
      }
      Self::PublicCity { world, coord } => {
        f.debug_struct("PublicCity")
          .field("world", world)
          .field("coord", coord)
          .finish()
      }
      Self::Report { world, report } => {
        f.debug_struct("Report")
          .field("world", world)
          .field("report", &report.id())
          .finish()
      }
      Self::Round { world, round } => {
        f.debug_struct("Round")
          .field("world", world)
          .field("round", &round.id())
          .finish()
      }
    }
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventTarget {
  Broadcast,
  Player(PlayerId),
}
