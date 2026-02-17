// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::VirtualMachine;
use crate::util::u8_array_to_usize;
use crate::value::Value;

impl VirtualMachine {
  pub(super) fn op_constant(&mut self) {
    let index = usize::from(unsafe { self.read_u8() });
    let constant = &self.chunk.constants()[index];
    unsafe { self.push(Value::from(constant)) };
  }

  pub(super) fn op_constant_long(&mut self) {
    let index = u8_array_to_usize(unsafe { [self.read_u8(), self.read_u8(), self.read_u8()] });
    let constant = &self.chunk.constants()[index];
    unsafe { self.push(Value::from(constant)) };
  }
}
