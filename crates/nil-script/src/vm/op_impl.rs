// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::VirtualMachine;
use crate::util::u8_array_to_usize;
use crate::value::Value;

impl VirtualMachine {
  pub(super) fn op_add(&mut self) {
    op_binary_number(self, |a, b| Value::Number(a + b));
  }

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

  pub(super) fn op_divide(&mut self) {
    op_binary_number(self, |a, b| Value::Number(a / b));
  }

  pub(super) fn op_multiply(&mut self) {
    op_binary_number(self, |a, b| Value::Number(a * b));
  }

  pub(super) fn op_negate(&mut self) {
    let value = self.pop();
    if let Value::Number(value) = value {
      self.push(Value::Number(-value));
    } else {
      todo!("err on invalid value");
    }
  }

  pub(super) fn op_return(&mut self) {
    self.pop();
  }

  pub(super) fn op_subtract(&mut self) {
    op_binary_number(self, |a, b| Value::Number(a - b));
  }
}

fn op_binary_number<F>(vm: &mut VirtualMachine, apply: F)
where
  F: FnOnce(f64, f64) -> Value,
{
  let b = vm.pop();
  let a = vm.pop();

  if let Value::Number(a) = a
    && let Value::Number(b) = b
  {
    vm.push(apply(a, b));
  } else {
    todo!("err on invalid value");
  }
}
