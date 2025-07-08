// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(
  iterator_try_collect,
  mixed_integer_ops_unsigned_sub,
  try_blocks,
  vec_deque_pop_if
)]

pub mod battle;
pub mod chat;
pub mod continent;
pub mod error;
pub mod event;
pub mod infrastructure;
pub mod lobby;
pub mod player;
pub mod resource;
pub mod round;
pub mod script;
pub mod unit;
pub mod village;
pub mod world;
