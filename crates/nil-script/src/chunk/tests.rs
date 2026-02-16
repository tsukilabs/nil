// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::chunk::Chunk;
use crate::op_code::OpCode::*;
use anyhow::Result;

#[test]
fn line_of() -> Result<()> {
  let mut chunk = Chunk::new();

  chunk.write_constant(5u16, 1)?;
  chunk.write_op(OP_RETURN, 1);
  chunk.write_op(OP_RETURN, 2);
  chunk.write_op(OP_RETURN, 2);
  chunk.write_op(OP_RETURN, 3);
  chunk.write_op(OP_RETURN, 4);

  assert_eq!(chunk.line_of(0)?, 1);
  assert_eq!(chunk.line_of(1)?, 1);
  assert_eq!(chunk.line_of(2)?, 1);
  assert_eq!(chunk.line_of(3)?, 2);
  assert_eq!(chunk.line_of(4)?, 2);
  assert_eq!(chunk.line_of(5)?, 3);
  assert_eq!(chunk.line_of(6)?, 4);

  Ok(())
}
