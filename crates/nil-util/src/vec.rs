// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use smallvec::SmallVec;
use std::ops::{ControlFlow, Try};

pub trait VecExt<T> {
  fn push_unique(&mut self, value: T) -> Option<T>
  where
    T: PartialEq;

  fn try_push<R>(&mut self, value: R)
  where
    R: Try<Output = T>;
}

macro_rules! impl_trait {
  () => {
    fn push_unique(&mut self, value: T) -> Option<T>
    where
      T: PartialEq,
    {
      if self.contains(&value) {
        Some(value)
      } else {
        self.push(value);
        None
      }
    }

    fn try_push<R>(&mut self, value: R)
    where
      R: Try<Output = T>,
    {
      if let ControlFlow::Continue(value) = value.branch() {
        self.push(value);
      }
    }
  };
}

impl<T> VecExt<T> for Vec<T> {
  impl_trait!();
}

impl<const N: usize, T> VecExt<T> for SmallVec<[T; N]> {
  impl_trait!();
}
