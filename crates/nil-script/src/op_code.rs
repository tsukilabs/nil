// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use std::mem;
use strum::Display;

pub const LONG_SIZE: usize = usize::from_le_bytes([255, 255, 255, 0, 0, 0, 0, 0]);

#[repr(u8)]
#[expect(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum OpCode {
  OP_CONSTANT,
  OP_CONSTANT_LONG,
  OP_RETURN,
}

impl OpCode {
  pub fn size(self) -> usize {
    match self {
      OpCode::OP_RETURN => 1,
      OpCode::OP_CONSTANT => 2,
      OpCode::OP_CONSTANT_LONG => 4,
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
