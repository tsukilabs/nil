// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::error::Result;
use crate::router;
use crate::server::spawn_round_duration_task;
use nil_core::world::World;
use nil_server_database::Database;
use std::net::SocketAddr;
use std::sync::Weak;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task::spawn;

pub async fn start(database_url: &str) -> Result<()> {
  let router = router::create()
    .with_state(App::new_remote(database_url)?)
    .into_make_service_with_connect_info::<SocketAddr>();

  let (listener, _) = super::bind(3000).await.unwrap();
  axum::serve(listener, router)
    .await
    .expect("Failed to start Call of Nil server");

  Ok(())
}

#[bon::builder]
pub(crate) fn on_next_round(
  database: Database,
  weak_world: Weak<RwLock<World>>,
  #[builder(into)] round_duration: Option<Duration>,
) -> Box<dyn Fn(&mut World) + Send + Sync> {
  Box::new(move |world: &mut World| {
    let id = world.id();
    let database = database.clone();

    world.save(move |bytes| {
      if let Err(err) = database.update_game_blob(id, &bytes) {
        tracing::error!(message = %err, error = ?err);
      }
    });

    if let Some(duration) = round_duration {
      let round = world.round().id();
      let weak_world = Weak::clone(&weak_world);
      spawn(spawn_round_duration_task(round, weak_world, duration));
    }
  })
}
