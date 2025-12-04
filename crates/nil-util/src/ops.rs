// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use std::ops::Try;

pub trait TryElse<T> {
  fn try_else<F, R>(self, f: F) -> R
  where
    F: FnOnce() -> R,
    R: Try<Output = T>;
}

impl<T> TryElse<T> for Option<T> {
  fn try_else<F, R>(self, f: F) -> R
  where
    F: FnOnce() -> R,
    R: Try<Output = T>,
  {
    match self {
      Self::Some(value) => R::from_output(value),
      Self::None => f(),
    }
  }
}

impl<T, E> TryElse<T> for Result<T, E> {
  fn try_else<F, R>(self, f: F) -> R
  where
    F: FnOnce() -> R,
    R: Try<Output = T>,
  {
    match self {
      Self::Ok(value) => R::from_output(value),
      Self::Err(..) => f(),
    }
  }
}
