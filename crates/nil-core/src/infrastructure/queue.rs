// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::resources::Workforce;
use std::collections::VecDeque;

pub trait InfrastructureQueue<T>
where
  T: InfrastructureQueueOrder,
{
  fn queue_mut(&mut self) -> &mut VecDeque<T>;

  /// Consumes workforce until it runs out or the entire queue is completed.
  fn process(&mut self, mut workforce: Workforce) -> Vec<T> {
    let mut orders = Vec::new();
    loop {
      if workforce == 0 {
        break;
      }

      match self
        .queue_mut()
        .pop_front_if(|order| order.consume(&mut workforce))
      {
        Some(order) => orders.push(order),
        None => break,
      }
    }

    if !orders.is_empty() {
      self.queue_mut().shrink_to_fit();
    }

    orders
  }
}

pub trait InfrastructureQueueOrder {
  fn is_done(&self) -> bool;
  fn set_done(&mut self);
  fn pending_workforce_mut(&mut self) -> Option<&mut Workforce>;

  fn consume(&mut self, workforce: &mut Workforce) -> bool {
    if let Some(pending) = self.pending_workforce_mut() {
      if *pending > 0 {
        let previous = *pending;
        *pending -= *workforce;

        // Decreases the available workforce based on the quantity consumed by this order.
        *workforce -= previous - *pending;
      }

      if *pending == 0 {
        self.set_done();
      }
    }

    self.is_done()
  }
}
