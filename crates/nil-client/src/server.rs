// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::ffi::{OsStr, OsString};
use std::net::SocketAddrV4;
use std::sync::{Arc, LazyLock};
use strum::EnumIs;
use url::Url;

static REMOTE_SERVER_ADDR: LazyLock<Box<str>> = LazyLock::new(|| {
  if let Ok(addr) = env::var("NIL_REMOTE_SERVER_ADDR") {
    Box::from(addr)
  } else {
    Box::from("tsukilabs.dev.br/nil")
  }
});

#[derive(Clone, Copy, Debug, Default, EnumIs, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ServerAddr {
  #[default]
  Remote,
  Local {
    addr: SocketAddrV4,
  },
}

impl ServerAddr {
  #[inline]
  pub fn url(&self, route: &str) -> Result<Url> {
    self.url_with_scheme("http", route)
  }

  pub fn url_with_scheme(&self, scheme: &str, route: &str) -> Result<Url> {
    let url = match self {
      Self::Remote => {
        let addr = REMOTE_SERVER_ADDR.as_str();
        format!("{scheme}://{addr}/{route}")
      }
      Self::Local { addr } => {
        let ip = addr.ip();
        let port = addr.port();
        format!("{scheme}://{ip}:{port}/{route}")
      }
    };

    Ok(Url::parse(&url)?)
  }
}

impl From<SocketAddrV4> for ServerAddr {
  fn from(addr: SocketAddrV4) -> Self {
    Self::Local { addr }
  }
}

impl From<&[u8]> for ServerAddr {
  fn from(bytes: &[u8]) -> Self {
    if let Ok(addr) = SocketAddrV4::parse_ascii(bytes) {
      Self::Local { addr }
    } else {
      Self::Remote
    }
  }
}

impl From<&OsStr> for ServerAddr {
  fn from(value: &OsStr) -> Self {
    Self::from(value.as_encoded_bytes())
  }
}

impl From<OsString> for ServerAddr {
  fn from(value: OsString) -> Self {
    Self::from(value.as_os_str())
  }
}

macro_rules! from_bytes {
  ($($type_:ty),+ $(,)?) => {
    $(
      impl From<$type_> for ServerAddr {
        fn from(value: $type_) -> Self {
          Self::from(value.as_bytes())
        }
      }
    )+
  };
}

from_bytes!(&str, String, &String, Arc<str>, Box<str>);
