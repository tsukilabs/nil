// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use nil_script::chunk::Chunk;
use nil_script::debug;
use nil_script::op_code::OpCode::*;

fn main() -> Result<()> {
  let mut chunk = Chunk::new();

  chunk.write_constant(5u16, 1);
  chunk.write_op(OP_RETURN, 1);
  chunk.write_op(OP_RETURN, 2);
  chunk.write_op(OP_RETURN, 2);
  chunk.write_op(OP_RETURN, 3);
  chunk.write_op(OP_RETURN, 4);

  debug::disassemble(&chunk)?;

  Ok(())
}
