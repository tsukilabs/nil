// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]

pub mod growth;
pub mod ops;
pub mod roman;

pub use nil_num_macros::{BigIntU64, BigIntUsize};
