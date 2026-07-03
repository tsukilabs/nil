// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use diesel::Connection;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub(super) fn run_pending_migrations(url: &str) -> Result<()> {
  SqliteConnection::establish(url)?
    .run_pending_migrations(MIGRATIONS)
    .map_err(Error::MigrationFailed)?;

  Ok(())
}
