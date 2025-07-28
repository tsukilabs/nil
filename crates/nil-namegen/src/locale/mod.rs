// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod en_us;
mod pt_br;

use crate::adjective::Adjective;
use crate::gender::Gender;
use crate::name::Name;
use rand::random_range;
use rand::seq::IndexedRandom;

pub(crate) struct Locale {
  language: Language,
  female: Vec<&'static str>,
  male: Vec<&'static str>,
  adjective: Vec<Adjective>,
}

macro_rules! collect {
  ($language:expr, $module:ident) => {{
    Locale {
      language: $language,
      female: $module::FEMALE.to_vec(),
      male: $module::MALE.to_vec(),
      adjective: $module::ADJECTIVE.to_vec(),
    }
  }};
}

impl Locale {
  pub fn new(language: Language) -> Self {
    match language {
      Language::English => collect!(language, en_us),
      Language::Portuguese => collect!(language, pt_br),
    }
  }

  pub fn generate(&mut self, gender: Gender) -> Option<Name> {
    let name = Name::builder()
      .base(self.take_name(gender)?)
      .adjective(self.take_adjective()?)
      .gender(gender)
      .language(self.language)
      .build();

    Some(name)
  }

  fn take_name(&mut self, gender: Gender) -> Option<&'static str> {
    let pool = match gender {
      Gender::Female => &mut self.female,
      Gender::Male => &mut self.male,
    };

    take(pool)
  }

  fn take_adjective(&mut self) -> Option<Adjective> {
    if self.adjective.is_empty() {
      let adjective = match self.language {
        Language::English => en_us::ADJECTIVE,
        Language::Portuguese => pt_br::ADJECTIVE,
      };

      self.adjective = adjective.to_vec();
      self
        .adjective
        .choose(&mut rand::rng())
        .copied()
    } else {
      take(&mut self.adjective)
    }
  }
}

fn take<T>(pool: &mut Vec<T>) -> Option<T> {
  if pool.is_empty() {
    None
  } else {
    let idx = random_range(0..pool.len());
    Some(pool.swap_remove(idx))
  }
}

impl Default for Locale {
  fn default() -> Self {
    Self::new(Language::default())
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Language {
  #[default]
  English,
  Portuguese,
}
