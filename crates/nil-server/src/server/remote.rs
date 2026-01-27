// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::error::Result;
use crate::router;
use std::net::SocketAddr;

pub async fn start_remote(database_url: &str) -> Result<()> {
  let router = router::create()
    .with_state(App::new_remote(database_url)?)
    .into_make_service_with_connect_info::<SocketAddr>();

  let (listener, _) = super::bind(63200).await.unwrap();
  axum::serve(listener, router)
    .await
    .expect("Failed to start Call of Nil server");

  Ok(())
}
