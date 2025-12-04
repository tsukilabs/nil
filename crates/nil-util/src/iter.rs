// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub trait IterExt: Iterator {
  fn collect_map<K, V>(self) -> HashMap<K, V>
  where
    Self: Sized + Iterator<Item = (K, V)>,
    K: Hash + Eq,
  {
    self.collect()
  }

  fn collect_deque(self) -> VecDeque<Self::Item>
  where
    Self: Sized,
  {
    self.collect()
  }

  fn collect_set(self) -> HashSet<Self::Item>
  where
    Self: Sized,
    Self::Item: Hash + Eq,
  {
    self.collect()
  }
}

impl<T: Iterator> IterExt for T {}
