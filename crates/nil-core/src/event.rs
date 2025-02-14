use crate::player::Player;
use crate::round::RoundState;
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
  pub(crate) fn emit(&self, event: Event) {
    let _ = self.sender.send(Bytes::from(event));
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
  RoundUpdated { round: RoundState },
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
