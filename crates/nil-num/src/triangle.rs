// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use num_traits::ToPrimitive;

#[inline]
pub const fn triangle(n: u32) -> u32 {
  n.saturating_mul(n + 1) / 2
}

pub fn nearest_triangle(value: impl Into<f64>) -> u32 {
  let value: f64 = value.into();
  let delta = 1.0 + (8.0 * value);
  if !delta.is_finite() || delta < 0.0 {
    return 0;
  }

  let x = (-1.0 + delta.sqrt()) / 2.0;
  let x = x.floor().max(0.0);
  x.to_u32().unwrap_or(0)
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

  #[test]
  fn nearest_triangle() {
    assert_eq!(super::nearest_triangle(0u32), 0);
    assert_eq!(super::nearest_triangle(1u32), 1);
    assert_eq!(super::nearest_triangle(2u32), 1);
    assert_eq!(super::nearest_triangle(3u32), 2);
    assert_eq!(super::nearest_triangle(4u32), 2);
    assert_eq!(super::nearest_triangle(5u32), 2);
    assert_eq!(super::nearest_triangle(6u32), 3);
    assert_eq!(super::nearest_triangle(7u32), 3);
    assert_eq!(super::nearest_triangle(8u32), 3);
    assert_eq!(super::nearest_triangle(9u32), 3);
    assert_eq!(super::nearest_triangle(10u32), 4);

    assert_eq!(super::nearest_triangle(-1i32), 0);
    assert_eq!(super::nearest_triangle(-1.0f64), 0);

    assert_eq!(super::nearest_triangle(f64::NAN), 0);
    assert_eq!(super::nearest_triangle(f64::INFINITY), 0);
    assert_eq!(super::nearest_triangle(f64::NEG_INFINITY), 0);
  }
}
