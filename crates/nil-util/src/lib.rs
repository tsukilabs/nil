mod error;
mod task;

pub use error::{Error, Result};
pub use task::spawn;

use bytes::Bytes;
use serde::Serialize;

pub fn to_bytes<T>(value: &T) -> Result<Bytes>
where
  T: ?Sized + Serialize,
{
  serde_json::to_vec(value)
    .map(Bytes::from)
    .map_err(Into::into)
}
