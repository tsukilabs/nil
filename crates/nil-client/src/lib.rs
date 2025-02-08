#![feature(let_chains, try_blocks)]

mod client;
mod error;

pub use client::Client;
pub use error::{Error, Result};
