// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::VirtualMachine;
use crate::util::u8_array_to_usize;
use crate::value::Value;

impl VirtualMachine {
  pub(super) fn op_constant(&mut self) {
    let index = usize::from(self.read_u8());
    let constant = &self.chunk.constants()[index];
    self.push(Value::from(constant));
  }

  pub(super) fn op_constant_long(&mut self) {
    let index = u8_array_to_usize([self.read_u8(), self.read_u8(), self.read_u8()]);
    let constant = &self.chunk.constants()[index];
    self.push(Value::from(constant));
  }

  pub(super) fn op_negate(&mut self) {
    let value = self.pop();
    if let Value::Number(value) = &value {
      self.push(Value::Number(-value));
    } else {
      todo!("err on invalid value");
    }
  }

  pub(super) fn op_return(&mut self) {
    self.pop();
  }
}
