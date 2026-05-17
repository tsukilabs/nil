// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(exit_status_error, try_trait_v2)]

#[cfg(feature = "iter")]
pub mod iter;

#[cfg(feature = "ops")]
pub mod ops;

#[cfg(feature = "process")]
pub mod process;

#[cfg(feature = "string")]
pub mod string;

#[cfg(feature = "vec")]
pub mod vec;

#[cfg(feature = "macros")]
pub use nil_util_macros::{BigInt, ConstDeref, F64Math};
