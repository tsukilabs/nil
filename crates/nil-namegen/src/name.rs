// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::adjective::Adjective;
use crate::gender::Gender;
use crate::locale::Language;
use bon::Builder;
use std::fmt;
use std::sync::Arc;

#[derive(Builder, Clone, Debug)]
pub struct Name {
  base: &'static str,
  adjective: Adjective,
  gender: Gender,
  language: Language,
}

impl Name {
  pub fn to_arc(self) -> Arc<str> {
    Arc::from(self.to_string())
  }
}

impl fmt::Display for Name {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let base = self.base;
    let adjective = self.adjective.as_str(self.gender);

    match self.language {
      Language::English => {
        write!(f, "{base} the {adjective}")
      }
      Language::Portuguese => {
        match self.gender {
          Gender::Female => write!(f, "{base}, a {adjective}"),
          Gender::Male => write!(f, "{base}, o {adjective}"),
        }
      }
    }
  }
}
