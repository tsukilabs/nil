// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(iterator_try_collect, try_blocks, vec_deque_pop_if)]

pub mod battle;
pub mod chat;
pub mod city;
pub mod continent;
pub mod error;
pub mod ethic;
pub mod event;
pub mod infrastructure;
pub mod military;
pub mod npc;
pub mod player;
pub mod resources;
pub mod round;
pub mod savedata;
pub mod script;
pub mod world;
