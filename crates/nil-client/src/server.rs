// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::net::SocketAddrV4;
use strum::EnumIs;
use url::Url;

#[cfg(debug_assertions)]
const REMOTE_SERVER_ADDR: &str = "127.0.0.0:3000";
#[cfg(not(debug_assertions))]
const REMOTE_SERVER_ADDR: &str = "tsukilabs.dev.br/nil";

#[derive(Clone, Copy, Debug, EnumIs, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ServerAddr {
  Local { addr: SocketAddrV4 },
  Remote,
}

impl ServerAddr {
  #[inline]
  pub fn url(&self, route: &str) -> Result<Url> {
    self.url_with_scheme("http", route)
  }

  pub fn url_with_scheme(&self, scheme: &str, route: &str) -> Result<Url> {
    let url = match self {
      Self::Remote => format!("{scheme}://{REMOTE_SERVER_ADDR}/{route}"),
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
