use serde::Serialize;
use serde::ser::Serializer;
use std::fmt::Display;

pub use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("failed to connect websocket: {reason}")]
  FailedToConnectWebsocket { reason: String },
  #[error("failed to decode response: {reason}")]
  FailedToDecodeResponse { reason: String },
  #[error("request failed: {reason}")]
  RequestFailed { reason: String },
}

impl Error {
  pub(crate) fn failed_to_decode(reason: impl Display) -> Self {
    Self::FailedToDecodeResponse { reason: reason.to_string() }
  }

  pub(crate) fn request_failed(reason: impl Display) -> Self {
    Self::RequestFailed { reason: reason.to_string() }
  }
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_str())
  }
}

impl From<Error> for String {
  fn from(value: Error) -> Self {
    value.to_string()
  }
}
