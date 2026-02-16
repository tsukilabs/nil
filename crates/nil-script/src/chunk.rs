// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::op_code::{LONG_SIZE, OpCode};
use crate::value::Constant;
use std::fmt;
use std::ops::Index;

#[derive(Default)]
pub struct Chunk {
  code: Vec<u8>,
  lines: Vec<Line>,
  constants: Vec<Constant>,
}

impl Chunk {
  #[inline]
  pub fn new() -> Self {
    Self::default()
  }

  pub(crate) fn constants(&self) -> &[Constant] {
    &self.constants
  }

  fn write(&mut self, byte: u8, line: usize) {
    self.code.push(byte);

    if let Some(last) = self.lines.last_mut()
      && last.value == line
    {
      last.repeat = last.repeat.strict_add(1);
    } else {
      self.lines.push(Line::new(line));
    }
  }

  #[inline]
  pub fn write_op(&mut self, op: OpCode, line: usize) {
    self.write(op as u8, line);
  }

  pub fn write_constant<T>(&mut self, constant: T, line: usize)
  where
    T: Into<Constant>,
  {
    let index = self.add_constant(constant);
    let bytes = index.to_le_bytes();

    if index > usize::from(u8::MAX) {
      self.write_op(OpCode::OP_CONSTANT_LONG, line);
      self.write(bytes[0], line);
      self.write(bytes[1], line);
      self.write(bytes[2], line);
    } else {
      self.write_op(OpCode::OP_CONSTANT, line);
      self.write(bytes[0], line);
    }
  }

  fn add_constant<T>(&mut self, constant: T) -> usize
  where
    T: Into<Constant>,
  {
    let next_offset = self.constants.len();
    if next_offset > LONG_SIZE {
      todo!("err");
    }

    self.constants.push(constant.into());
    next_offset
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.code.len()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.code.is_empty()
  }

  pub fn line_of(&self, offset: usize) -> Result<&Line> {
    let mut acc = offset;
    for line in &self.lines {
      if acc < line.repeat {
        return Ok(line);
      }

      acc -= line.repeat;
    }

    Err(Error::LineNotFound { offset })
  }
}

impl Index<usize> for Chunk {
  type Output = u8;

  fn index(&self, offset: usize) -> &Self::Output {
    &self.code[offset]
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
  value: usize,
  repeat: usize,
}

impl Line {
  #[inline]
  pub const fn new(line: usize) -> Self {
    Self { value: line, repeat: 1 }
  }
}

impl fmt::Display for Line {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.value)
  }
}
