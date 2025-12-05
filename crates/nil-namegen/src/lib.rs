// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use itertools::Itertools;
use rand::random_range;
use rand::seq::IndexedRandom;
use std::sync::Arc;
use strum::VariantArray;

const FEMALE: &str = include_str!("../data/female.csv");
const MALE: &str = include_str!("../data/male.csv");
const SURNAME: &str = include_str!("../data/surname.csv");

pub fn generate(count: usize) -> Vec<Arc<str>> {
  let mut names = Vec::with_capacity(count);
  let mut female = lines(FEMALE);
  let mut male = lines(MALE);
  let mut surname = lines(SURNAME);

  for _ in 0..=count {
    let gender = Gender::random();
    let Some(first) = (match gender {
      Gender::Female => take(&mut female),
      Gender::Male => take(&mut male),
    }) else {
      break;
    };

    let Some(last) = take(&mut surname) else { break };
    if first != last {
      names.push(Arc::from(format!("{first} {last}")));
    }
  }

  names.into_iter().unique().collect()
}

fn lines(file: &'static str) -> Vec<&'static str> {
  file
    .trim()
    .lines()
    .map(str::trim)
    .filter(|line| !line.is_empty())
    .unique()
    .collect()
}

fn take(pool: &mut Vec<&'static str>) -> Option<&'static str> {
  if pool.is_empty() {
    None
  } else {
    let idx = random_range(0..pool.len());
    Some(pool.swap_remove(idx))
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, VariantArray)]
enum Gender {
  Female,
  Male,
}

impl Gender {
  fn random() -> Self {
    Self::VARIANTS
      .choose(&mut rand::rng())
      .copied()
      .expect("Self::VARIANTS should never be empty")
  }
}
