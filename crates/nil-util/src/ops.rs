// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use std::ops::{ControlFlow, Try};

pub trait TryElse<T> {
  fn unwrap_or_try_else<F, R>(self, f: F) -> R
  where
    F: FnOnce() -> R,
    R: Try<Output = T>;
}

impl<T, U> TryElse<T> for U
where
  U: Try<Output = T>,
{
  fn unwrap_or_try_else<F, R>(self, f: F) -> R
  where
    F: FnOnce() -> R,
    R: Try<Output = T>,
  {
    match self.branch() {
      ControlFlow::Continue(value) => R::from_output(value),
      ControlFlow::Break(..) => f(),
    }
  }
}
