// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub trait WrapSome<T> {
  fn wrap_some(self) -> Option<T>;
}

impl<T> WrapSome<T> for T {
  fn wrap_some(self) -> Option<T> {
    Some(self)
  }
}
