// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use serde::{Deserialize, Serialize};
use std::fmt;
use std::iter::repeat_n;
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Roman(Box<[Numeral]>);

impl Roman {
  const MIN: usize = 1;
  const MAX: usize = 3999;

  pub fn parse(value: impl ToRoman) -> Option<Self> {
    value.to_roman()
  }
}

impl Default for Roman {
  fn default() -> Self {
    Self(Box::from([Numeral::I]))
  }
}

impl fmt::Display for Roman {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for numeral in &self.0 {
      write!(f, "{numeral}")?;
    }

    Ok(())
  }
}

impl From<&Roman> for u16 {
  fn from(roman: &Roman) -> Self {
    let mut value = 0u16;
    for numeral in &roman.0 {
      let numeral = u16::from(*numeral);
      value = value.saturating_add(numeral);
    }

    value
  }
}

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Deserialize, Serialize, EnumIter)]
#[serde(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
pub enum Numeral {
  I,
  IV,
  V,
  IX,
  X,
  XL,
  L,
  XC,
  C,
  CD,
  D,
  CM,
  M,
}

impl From<Numeral> for u16 {
  fn from(numeral: Numeral) -> Self {
    match numeral {
      Numeral::I => 1,
      Numeral::IV => 4,
      Numeral::V => 5,
      Numeral::IX => 9,
      Numeral::X => 10,
      Numeral::XL => 40,
      Numeral::L => 50,
      Numeral::XC => 90,
      Numeral::C => 100,
      Numeral::CD => 400,
      Numeral::D => 500,
      Numeral::CM => 900,
      Numeral::M => 1000,
    }
  }
}

macro_rules! impl_from_numeral {
  ($($target:ident),+ $(,)?) => {
    $(
      impl From<Numeral> for $target {
        fn from(numeral: Numeral) -> Self {
          u16::from(numeral).into()
        }
      }
    )+
  };
}

impl_from_numeral!(i32, i64, u32, u64, usize);

pub trait ToRoman {
  fn to_roman(self) -> Option<Roman>;
}

impl ToRoman for usize {
  fn to_roman(mut self) -> Option<Roman> {
    if (Roman::MIN..=Roman::MAX).contains(&self) {
      let mut roman = Vec::new();
      for numeral in Numeral::iter().rev() {
        if self == 0 {
          break;
        }

        let value = usize::from(numeral);
        let count = self.saturating_div(value);
        roman.extend(repeat_n(numeral, count));
        self = self.saturating_sub(count * value);
      }

      Some(Roman(roman.into_boxed_slice()))
    } else {
      None
    }
  }
}

macro_rules! impl_to_roman {
  (signed @ $($num:ident),+ $(,)?) => {
    $(
      impl ToRoman for $num {
        fn to_roman(self) -> Option<Roman> {
          (self as usize).to_roman()
        }
      }
    )+
  };
  (unsigned @ $($num:ident),+ $(,)?) => {
    $(
      impl ToRoman for $num {
        fn to_roman(self) -> Option<Roman> {
          self.unsigned_abs().to_roman()
        }
      }
    )+
  };
}

impl_to_roman!(signed @ u8, u16, u32, u64, u128);
impl_to_roman!(unsigned @ i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
  use super::{Roman, ToRoman};

  macro_rules! to_str {
    ($number:expr) => {
      $number
        .to_roman()
        .unwrap()
        .to_string()
        .as_str()
    };
  }

  #[test]
  fn to_roman() {
    assert_eq!(to_str!(1), "I");
    assert_eq!(to_str!(4), "IV");
    assert_eq!(to_str!(5), "V");
    assert_eq!(to_str!(9), "IX");
    assert_eq!(to_str!(10), "X");
    assert_eq!(to_str!(30), "XXX");
    assert_eq!(to_str!(40), "XL");
    assert_eq!(to_str!(50), "L");
    assert_eq!(to_str!(100), "C");
    assert_eq!(to_str!(300), "CCC");
    assert_eq!(to_str!(400), "CD");
    assert_eq!(to_str!(500), "D");
    assert_eq!(to_str!(900), "CM");
    assert_eq!(to_str!(1000), "M");
    assert_eq!(to_str!(2350), "MMCCCL");
    assert_eq!(to_str!(3000), "MMM");
    assert_eq!(to_str!(3999), "MMMCMXCIX");
  }

  #[test]
  fn min_max() {
    assert!(Roman::parse(0u16).is_none());
    assert!(Roman::parse(2000u16).is_some());
    assert!(Roman::parse(4000u16).is_none());
  }
}
