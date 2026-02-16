// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::chunk::Chunk;
use crate::op_code::OpCode::{self, *};
use itertools::Itertools;
use std::iter::repeat_n;

pub fn disassemble(chunk: &Chunk) {
  let mut offset = 0;

  while offset < chunk.len() {
    offset = disassemble_instruction(chunk, offset);
  }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
  print!("{offset:04} ");

  let op = OpCode::from(chunk[offset]);
  let line = chunk.line_of(offset);

  if offset > 0 && line == chunk.line_of(offset - 1) {
    print!("   | ");
  } else {
    print!("{:04} ", line);
  }

  match op {
    OP_CONSTANT => constant(op, chunk, offset),
    OP_CONSTANT_LONG => constant_long(op, chunk, offset),
    OP_RETURN => simple(op),
  }

  offset.strict_add(op.size())
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
  let bytes = [chunk[offset + 1], chunk[offset + 2], chunk[offset + 3]]
    .into_iter()
    .chain(repeat_n(0u8, 5))
    .collect_array::<8>()
    .expect("iterator has exactly eight values");

  let index = usize::from_le_bytes(bytes);
  let constant = &chunk.constants()[index];
  println!("{op:16} {index:4} '{constant}'");
}
