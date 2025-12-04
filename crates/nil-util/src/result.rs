// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub trait WrapOk<T, E> {
  fn wrap_ok(self) -> Result<T, E>;
}

impl<T, E> WrapOk<T, E> for T {
  fn wrap_ok(self) -> Result<T, E> {
    Ok(self)
  }
}
