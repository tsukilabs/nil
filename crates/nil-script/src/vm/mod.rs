// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod op_impl;

use crate::chunk::Chunk;
use crate::debug;
use crate::error::Result;
use crate::op_code::OpCode::{self, *};
use crate::value::Value;
use std::ptr;

const STACK_SIZE: usize = 256;

pub struct VirtualMachine {
  chunk: Chunk,
  ip: *const u8,
  stack: Box<[Value; STACK_SIZE]>,
  stack_top: *mut Value,
}

impl VirtualMachine {
  pub fn new(mut chunk: Chunk) -> Self {
    if chunk.is_empty() {
      chunk.write_op(OP_RETURN, 1);
    }

    Self {
      chunk,
      ip: ptr::null(),
      stack: Box::new([const { Value::Nil }; STACK_SIZE]),
      stack_top: ptr::null_mut(),
    }
  }

  pub fn interpret(self) -> Result<()> {
    self.run()
  }

  fn run(mut self) -> Result<()> {
    self.ip = self.chunk.as_ptr();
    self.stack_top = self.stack.as_mut_ptr();

    loop {
      if cfg!(feature = "trace") {
        let offset = unsafe { self.ip.offset_from(self.chunk.as_ptr()) };
        debug::disassemble_instruction(&self.chunk, offset.try_into().unwrap())?;
      }

      match OpCode::from(self.read_u8()) {
        OP_ADD => self.op_add(),
        OP_CONSTANT => self.op_constant(),
        OP_CONSTANT_LONG => self.op_constant_long(),
        OP_DIVIDE => self.op_divide(),
        OP_MULTIPLY => self.op_multiply(),
        OP_NEGATE => self.op_negate(),
        OP_SUBTRACT => self.op_subtract(),

        OP_RETURN => {
          self.op_return();
          return Ok(());
        }
      }
    }
  }

  fn read_u8(&mut self) -> u8 {
    let byte = unsafe { *self.ip };
    self.ip = unsafe { self.ip.add(1) };
    byte
  }

  fn push(&mut self, value: Value) {
    unsafe { *self.stack_top = value };
    self.stack_top = unsafe { self.stack_top.add(1) };
  }

  fn pop(&mut self) -> Value {
    self.stack_top = unsafe { self.stack_top.sub(1) };
    // unsafe { *self.stack_top }
    todo!()
  }
}
