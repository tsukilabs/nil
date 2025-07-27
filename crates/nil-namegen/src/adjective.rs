// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::gender::Gender;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Adjective {
  Neutral(&'static str),
  Gendered {
    female: &'static str,
    male: &'static str,
  },
}

impl Adjective {
  pub fn as_str(&self, gender: Gender) -> &str {
    match self {
      Adjective::Neutral(adj) => adj,
      Adjective::Gendered { female, male } => {
        match gender {
          Gender::Female => female,
          Gender::Male => male,
        }
      }
    }
  }
}

#[doc(hidden)]
#[macro_export]
macro_rules! a {
  ($neutral:literal) => {{ $crate::adjective::Adjective::Neutral($neutral) }};
  ($f:literal, $m:literal) => {{ $crate::adjective::Adjective::Gendered { female: $f, male: $m } }};
}
