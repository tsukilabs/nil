// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub trait StringExt {
  fn truncate_chars(&mut self, max: usize);
}

impl StringExt for String {
  fn truncate_chars(&mut self, max: usize) {
    let chars = self.chars().count();
    let excess = chars.saturating_sub(max);
    if excess > 0 {
      for _ in 0..excess {
        self.pop();
      }
    }
  }
}
