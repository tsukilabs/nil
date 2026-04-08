// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub(super) fn run_pending_migrations(conn: &mut SqliteConnection) -> Result<()> {
  conn
    .run_pending_migrations(MIGRATIONS)
    .map_err(Error::MigrationFailed)?;

  Ok(())
}
