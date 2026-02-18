// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(nonpoison_mutex, str_as_str, sync_nonpoison, try_blocks)]

mod database;
pub mod error;
mod migration;
pub mod model;
mod schema;
pub mod sql_types;

pub use database::Database;
