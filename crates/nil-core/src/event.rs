// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::chat::ChatMessage;
use crate::lobby::LobbyState;
use crate::player::{Player, PlayerId, PlayerStatus};
use crate::round::Round;
use crate::village::{Coord, PublicVillage};
use bytes::Bytes;
use nil_util::serde::{from_slice, to_bytes};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::Display;
use tokio::sync::broadcast::{Receiver, Sender, channel};

#[cfg(debug_assertions)]
use tracing::info;

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
    info!(?target, ?event);

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
  ChatMessage {
    message: ChatMessage,
  },
  GuestLeft {
    guest: Player,
  },
  LobbyUpdated {
    lobby: LobbyState,
  },
  PlayerSpawned {
    player: Player,
  },
  PlayerStatusUpdated {
    player: PlayerId,
    status: PlayerStatus,
  },
  PlayerUpdated {
    player: PlayerId,
  },
  RoundUpdated {
    round: Round,
  },
  VillageSpawned {
    village: PublicVillage,
  },
  VillageUpdated {
    coord: Coord,
  },
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
