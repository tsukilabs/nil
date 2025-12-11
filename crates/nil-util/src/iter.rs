// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::ops::{ControlFlow, Try};

pub trait IterExt: Iterator {
  fn collect_deque(self) -> VecDeque<Self::Item>
  where
    Self: Sized,
  {
    self.collect()
  }

  fn collect_map<K, V>(self) -> HashMap<K, V>
  where
    Self: Sized + Iterator<Item = (K, V)>,
    K: Hash + Eq,
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

  fn try_each<R>(self) -> R
  where
    Self: Iterator<Item = R> + Sized,
    R: Try<Output = ()>,
  {
    for item in self {
      if let ControlFlow::Break(e) = item.branch() {
        return R::from_residual(e);
      }
    }

    R::from_output(())
  }
}

impl<T: Iterator> IterExt for T {}
