use crate::error::{Result, StdResult};
use crate::player::Player;
use crate::turn::Turn;
use bytes::Bytes;
use nil_util::to_bytes;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use std::fmt;
use strum::Display;
use tokio::sync::broadcast::{Receiver, Sender, channel};

pub type Listener = Receiver<Bytes>;

#[derive(Clone)]
pub(crate) struct Emitter {
  sender: Sender<Bytes>,
}

impl Emitter {
  pub(crate) fn emit(&self, event: Event) -> Result<()> {
    let _ = self.sender.send(Bytes::try_from(event)?);
    Ok(())
  }

  pub(crate) fn subscribe(&self) -> Listener {
    self.sender.subscribe()
  }
}

impl Default for Emitter {
  fn default() -> Self {
    let (sender, _) = channel(100);
    Self { sender }
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
  PlayerJoined { player: Player },
  TurnUpdated { turn: Turn },
}

impl TryFrom<Event> for Bytes {
  type Error = EventError;

  fn try_from(event: Event) -> StdResult<Self, Self::Error> {
    to_bytes(&event).map_err(EventError::Serialize)
  }
}

impl TryFrom<Bytes> for Event {
  type Error = EventError;

  fn try_from(bytes: Bytes) -> StdResult<Self, Self::Error> {
    from_slice(&bytes).map_err(EventError::Deserialize)
  }
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
#[remain::sorted]
pub enum EventError {
  #[error("failed to deserialize event: {0}")]
  Deserialize(#[source] serde_json::Error),
  #[error("failed to serialize event: {0}")]
  Serialize(#[source] nil_util::Error),
}
