// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod local;
pub mod remote;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::net::TcpListener;

async fn bind(port: u16) -> Option<(TcpListener, SocketAddrV4)> {
  let result = try {
    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let listener = TcpListener::bind(addr).await?;
    let SocketAddr::V4(addr) = listener.local_addr()? else {
      unreachable!("Address should never be Ipv6");
    };

    tracing::info!("Listening on port {}", addr.port());

    (listener, addr)
  };

  result.ok()
}
