// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod line;

#[cfg(test)]
mod tests;

use crate::error::{Error, Result};
use crate::op_code::OpCode;
use crate::value::Constant;
use line::Line;
use std::ops::Index;

const U24: usize = usize::from_le_bytes([255, 255, 255, 0, 0, 0, 0, 0]);

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

  pub fn write_constant<T>(&mut self, constant: T, line: usize) -> Result<()>
  where
    T: Into<Constant>,
  {
    let index = self.add_constant(constant)?;
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

    Ok(())
  }

  fn add_constant<T>(&mut self, constant: T) -> Result<usize>
  where
    T: Into<Constant>,
  {
    let next_offset = self.constants.len();
    if next_offset <= U24 {
      self.constants.push(constant.into());
      Ok(next_offset)
    } else {
      // A bigger offset would require four bytes.
      Err(Error::TooManyConstants)
    }
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.code.len()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.code.is_empty()
  }

  pub(crate) fn line_of(&self, offset: usize) -> Result<&Line> {
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
