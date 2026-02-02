// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::resources::workforce::Workforce;
use std::collections::VecDeque;

pub trait InfrastructureQueue<T>
where
  T: InfrastructureQueueOrder,
{
  fn queue(&self) -> &VecDeque<T>;
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

  fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
  where
    T: 'a,
  {
    self.queue().iter()
  }

  fn len(&self) -> usize {
    self.queue().len()
  }

  fn is_empty(&self) -> bool {
    self.queue().is_empty()
  }

  fn sum_pending_workforce(&self) -> Workforce {
    self
      .iter()
      .filter_map(InfrastructureQueueOrder::pending_workforce)
      .map(u32::from)
      .sum::<u32>()
      .into()
  }
}

pub trait InfrastructureQueueOrder {
  fn is_done(&self) -> bool;
  fn set_done(&mut self);

  fn pending_workforce(&self) -> Option<Workforce>;
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
