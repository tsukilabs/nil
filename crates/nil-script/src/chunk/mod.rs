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
  ops: Vec<u8>,
  lines: Vec<Line>,
  constants: Vec<Constant>,
}

impl Chunk {
  #[inline]
  pub fn new() -> Self {
    Self::default()
  }

  #[inline]
  pub fn constants(&self) -> &[Constant] {
    &self.constants
  }

  /// Returns a raw pointer to the [`OpCode`](crate::op_code::OpCode) buffer.
  #[inline]
  pub(crate) fn as_ptr(&self) -> *const u8 {
    self.ops.as_ptr()
  }

  fn write(&mut self, byte: u8, line: usize) {
    self.ops.push(byte);

    if let Some(last) = self.lines.last_mut()
      && last.value == line
    {
      last.count = last.count.strict_add(1);
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
    let offset = self.add_constant(constant)?;
    let bytes = offset.to_le_bytes();

    if offset > usize::from(u8::MAX) {
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
    let constant: Constant = constant.into();
    if let Some(offset) = self
      .constants
      .iter()
      .position(|it| it == &constant)
    {
      return Ok(offset);
    }

    let next_offset = self.constants.len();
    if next_offset <= U24 {
      self.constants.push(constant);
      Ok(next_offset)
    } else {
      // A bigger offset would require four bytes.
      Err(Error::TooManyConstants)
    }
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.ops.len()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.ops.is_empty()
  }

  pub(crate) fn line_of(&self, offset: usize) -> Result<&Line> {
    let mut acc = offset;
    for line in &self.lines {
      if acc < line.count {
        return Ok(line);
      }

      acc -= line.count;
    }

    Err(Error::LineNotFound { offset })
  }
}

impl Index<usize> for Chunk {
  type Output = u8;

  fn index(&self, offset: usize) -> &Self::Output {
    &self.ops[offset]
  }
}
