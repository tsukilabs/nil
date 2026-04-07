// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[inline]
pub fn triangle(n: u32) -> u32 {
  n.saturating_mul(n + 1) / 2
}

#[cfg(test)]
mod tests {
  #[test]
  fn triangle() {
    assert_eq!(super::triangle(0), 0);
    assert_eq!(super::triangle(1), 1);
    assert_eq!(super::triangle(2), 3);
    assert_eq!(super::triangle(3), 6);
    assert_eq!(super::triangle(4), 10);
    assert_eq!(super::triangle(5), 15);
    assert_eq!(super::triangle(6), 21);
    assert_eq!(super::triangle(7), 28);
    assert_eq!(super::triangle(8), 36);
    assert_eq!(super::triangle(9), 45);
    assert_eq!(super::triangle(10), 55);
  }
}
