// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]

mod adjective;
mod gender;
mod locale;
mod name;

use crate::gender::Gender;
use crate::locale::Locale;
use std::sync::Arc;

pub use locale::Language;
pub use name::Name;

pub fn generate(language: Language, count: usize) -> Vec<Name> {
  let mut locale = Locale::new(language);
  let mut names = Vec::with_capacity(count);

  for _ in 0..=count {
    let gender = Gender::random();
    let Some(name) = locale.generate(gender) else { break };
    names.push(name);
  }

  names
}

pub fn generate_arc(language: Language, count: usize) -> Vec<Arc<str>> {
  generate(language, count)
    .into_iter()
    .map(Name::to_arc)
    .collect()
}
