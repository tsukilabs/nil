// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::chunk::Chunk;
use crate::error::Result;
use crate::op_code::OpCode::{self, *};
use crate::util::u8_array_to_usize;

pub fn disassemble(chunk: &Chunk) -> Result<()> {
  let mut offset = 0;

  while offset < chunk.len() {
    offset = disassemble_instruction(chunk, offset)?;
  }

  Ok(())
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> Result<usize> {
  print!("{offset:04} ");

  let op = OpCode::from(chunk[offset]);
  let line = chunk.line_of(offset)?;

  if offset > 0 && line == chunk.line_of(offset - 1)? {
    print!("   | ");
  } else {
    print!("{:04} ", line.value());
  }

  match op {
    OP_CONSTANT => constant(op, chunk, offset),
    OP_CONSTANT_LONG => constant_long(op, chunk, offset),
    OP_RETURN => simple(op),
  }

  Ok(offset.strict_add(op.size()))
}

fn simple(op: OpCode) {
  println!("{op}");
}

fn constant(op: OpCode, chunk: &Chunk, offset: usize) {
  let index = usize::from(chunk[offset + 1]);
  let constant = &chunk.constants()[index];
  println!("{op:16} {index:4} '{constant}'");
}

fn constant_long(op: OpCode, chunk: &Chunk, offset: usize) {
  let index = u8_array_to_usize([chunk[offset + 1], chunk[offset + 2], chunk[offset + 3]]);
  let constant = &chunk.constants()[index];
  println!("{op:16} {index:4} '{constant}'");
}
