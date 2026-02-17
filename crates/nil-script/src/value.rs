// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use std::fmt;

#[derive(Default)]
pub enum Value {
  #[default]
  Nil,
  Bool(bool),
  Number(f64),
  String(String),
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Bool(it) => write!(f, "{it}"),
      Value::Nil => write!(f, "nil"),
      Value::Number(it) => write!(f, "{it}"),
      Value::String(it) => write!(f, "{it}"),
    }
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Constant {
  #[default]
  Nil,
  Number(f64),
  String(Box<str>),
}

impl fmt::Display for Constant {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Constant::Nil => write!(f, "nil"),
      Constant::Number(it) => write!(f, "{it}"),
      Constant::String(it) => write!(f, "{it}"),
    }
  }
}

impl From<&Constant> for Value {
  fn from(constant: &Constant) -> Self {
    match constant {
      Constant::Nil => Value::Nil,
      Constant::Number(it) => Value::Number(*it),
      Constant::String(it) => Value::String(it.to_string()),
    }
  }
}

macro_rules! from_num {
  ($enum_name:ident => $($num:ident),+ $(,)?) => {
    $(
      impl From<$num> for $enum_name {
        fn from(value: $num) -> Self {
          $enum_name::Number(f64::from(value))
        }
      }
    )+
  };
}

from_num!(Value => i8, i16, i32, u8, u16, u32, f32, f64);
from_num!(Constant => i8, i16, i32, u8, u16, u32, f32, f64);
