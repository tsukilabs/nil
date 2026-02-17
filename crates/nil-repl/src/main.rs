// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use nil_script::chunk::Chunk;
use nil_script::op_code::OpCode::*;
use nil_script::vm::VirtualMachine;

fn main() -> Result<()> {
  let mut chunk = Chunk::new();

  chunk.write_constant(5u16, 1)?;
  chunk.write_constant(5555u32, 1)?;
  chunk.write_constant(50u16, 1)?;
  chunk.write_constant(5u16, 1)?;
  chunk.write_op(OP_RETURN, 1);

  let vm = VirtualMachine::new(chunk);
  vm.interpret()?;

  Ok(())
}
