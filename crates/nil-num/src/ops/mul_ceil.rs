// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub trait MulCeil<Rhs = Self> {
  type Output;

  fn mul_ceil(self, rhs: Rhs) -> Self::Output;
}

impl MulCeil for f64 {
  type Output = f64;

  fn mul_ceil(self, rhs: f64) -> f64 {
    (self * rhs).ceil()
  }
}

#[macro_export]
macro_rules! impl_mul_ceil {
  ($($name:ident),+ $(,)?) => {
    $(
      impl $crate::ops::MulCeil<f64> for $name {
        type Output = f64;

        fn mul_ceil(self, rhs: f64) -> Self::Output {
          (f64::from(self) * rhs).ceil()
        }
      }

      impl $crate::ops::MulCeil<$name> for f64 {
        type Output = f64;

        fn mul_ceil(self, rhs: $name) -> Self::Output {
          (self * f64::from(rhs)).ceil()
        }
      }
    )+
  };
}

impl_mul_ceil!(i8, u8, i16, u16, i32, u32, f32);
