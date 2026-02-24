// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::error::Result;
use crate::router;
use nil_core::world::World;
use nil_server_database::Database;
use std::net::SocketAddr;

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

pub(crate) fn on_next_round(db: Database) -> Box<dyn Fn(&mut World) + Send + Sync> {
  Box::new(move |world: &mut World| {
    let db = db.clone();
    let id = world.config().id();

    world.save(move |bytes| {
      if let Err(err) = db.update_game_blob(id, &bytes) {
        tracing::error!(message = %err, error = ?err);
      }
    });
  })
}
