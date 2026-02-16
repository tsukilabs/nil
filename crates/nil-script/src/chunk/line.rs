// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
  pub(super) value: usize,
  pub(super) repeat: usize,
}

impl Line {
  #[inline]
  pub const fn new(line: usize) -> Self {
    Self { value: line, repeat: 1 }
  }

  #[inline]
  pub const fn value(&self) -> usize {
    self.value
  }
}

impl fmt::Display for Line {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.value)
  }
}

impl PartialEq<usize> for Line {
  fn eq(&self, other: &usize) -> bool {
    self.value.eq(other)
  }
}

impl PartialEq<usize> for &Line {
  fn eq(&self, other: &usize) -> bool {
    self.value.eq(other)
  }
}
