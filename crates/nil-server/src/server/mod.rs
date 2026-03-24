// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod local;
pub mod remote;

use crate::error::{AnyResult, Result};
use jiff::{SignedDuration, Zoned};
use nil_core::round::RoundId;
use nil_core::world::World;
use nil_server_types::round::RoundDuration;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::{Arc, Weak};
use std::time::Duration;
use tap::TryConv;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio::time::sleep;

async fn bind(port: u16) -> Result<(TcpListener, SocketAddrV4)> {
  let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
  let listener = TcpListener::bind(addr).await?;
  let SocketAddr::V4(addr) = listener.local_addr()? else {
    unreachable!("Address should never be Ipv6");
  };

  tracing::info!("Listening on port {}", addr.port());

  Ok((listener, addr))
}

async fn spawn_round_duration_task(
  current_round: RoundId,
  weak_world: Weak<RwLock<World>>,
  duration: RoundDuration,
) {
  if let Some(arc_world) = Weak::upgrade(&weak_world) {
    let result = try bikeshed AnyResult<()> {
      let delta = rand::random_range(1.0..=1.2);
      let duration = Duration::from(duration)
        .mul_f64(delta)
        .try_conv::<SignedDuration>()?;

      let lock = arc_world.read().await;

      // Always make sure to check the round id after locking.
      if lock.round().id() == current_round {
        let started_at = lock.round().started_at()?;
        let since = started_at.duration_until(&Zoned::now());

        drop(lock);

        if since >= duration {
          end_round(arc_world, current_round).await?;
        } else {
          // Don’t keep this around while sleeping.
          // Otherwise we may inadvertently prevent the world from being dropped.
          drop(arc_world);

          let until = duration.saturating_sub(since);
          sleep(Duration::try_from(until)?).await;

          if let Some(arc_world) = Weak::upgrade(&weak_world) {
            end_round(arc_world, current_round).await?;
          }
        }
      }
    };

    if let Err(err) = result {
      tracing::error!(message = %err, error = ?err);
    }
  }
}

async fn end_round(world: Arc<RwLock<World>>, current_round: RoundId) -> Result<()> {
  let mut world = world.write().await;
  let round = world.round();

  if round.id() == current_round && !round.is_idle() && world.has_any_active_player() {
    world.dangerously_end_round(true)?;
  }

  Ok(())
}
