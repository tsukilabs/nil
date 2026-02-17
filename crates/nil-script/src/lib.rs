// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![feature(variant_count)]

pub mod chunk;
pub mod compiler;
pub mod debug;
pub mod error;
pub mod op_code;
mod util;
pub mod value;
pub mod vm;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
