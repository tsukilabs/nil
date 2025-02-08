use serde::Serialize;
use serde::ser::Serializer;

pub use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Json(#[from] serde_json::Error),
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
