// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use std::mem;
use strum::Display;

#[repr(u8)]
#[expect(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum OpCode {
  OP_ADD,
  OP_CONSTANT,
  OP_CONSTANT_LONG,
  OP_DIVIDE,
  OP_MULTIPLY,
  OP_NEGATE,
  OP_RETURN,
  OP_SUBTRACT,
}

impl OpCode {
  #[expect(clippy::match_same_arms)]
  pub fn bytecode_size(self) -> u8 {
    use OpCode::*;
    match self {
      OP_ADD => 1,
      OP_CONSTANT => 2,
      OP_CONSTANT_LONG => 4,
      OP_DIVIDE => 1,
      OP_MULTIPLY => 1,
      OP_NEGATE => 1,
      OP_RETURN => 1,
      OP_SUBTRACT => 1,
    }
  }
}

impl From<u8> for OpCode {
  fn from(byte: u8) -> Self {
    assert!(
      usize::from(byte) <= (mem::variant_count::<OpCode>() - 1),
      "{byte} is not a valid op code"
    );

    unsafe { mem::transmute(byte) }
  }
}

impl From<OpCode> for u8 {
  fn from(code: OpCode) -> Self {
    code as u8
  }
}
