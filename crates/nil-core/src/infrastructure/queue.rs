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

#[doc(hidden)]
#[macro_export]
macro_rules! decl_recruit_queue {
  ($building:ident) => {
    paste::paste! {
      #[derive(Clone, Debug, Default, Deserialize, Serialize)]
      #[serde(rename_all = "camelCase")]
      pub struct [<$building RecruitQueue>] {
        orders: VecDeque<[<$building RecruitOrder>]>,
      }

      impl [<$building RecruitQueue>] {
        pub(crate) fn recruit(
          &mut self,
          request: &[<$building RecruitOrderRequest>],
          current_resources: Option<&Resources>,
        ) -> Result<&[<$building RecruitOrder>]> {
          let unit = UnitBox::from(request.unit);
          let chunk = unit.as_dyn().chunk();
          let size = SquadSize::new(chunk.size() * request.chunks);
          let resources = &chunk.resources() * request.chunks;
          let workforce = chunk.workforce() * request.chunks;

          if let Some(current_resources) = current_resources
            && current_resources
              .checked_sub(&resources)
              .is_none()
          {
            return Err(Error::InsufficientResources);
          }

          self.orders.push_back([<$building RecruitOrder>] {
            id: [<$building RecruitOrderId>]::new(),
            squad: Squad::new(unit.id(), size),
            resources,
            workforce,
            state: [<$building RecruitOrderState>]::new(workforce),
          });

          let len = self.orders.len();
          Ok(unsafe {
            self
              .orders
              .get(len.unchecked_sub(1))
              .unwrap_unchecked()
          })
        }

        /// Cancels a recruit order.
        #[must_use]
        pub(crate) fn cancel(&mut self, id: [<$building RecruitOrderId>]) -> Option<[<$building RecruitOrder>]> {
          let position = self
            .orders
            .iter()
            .position(|order| order.id == id)?;

          self.orders.remove(position)
        }
      }

      impl InfrastructureQueue<[<$building RecruitOrder>]> for [<$building RecruitQueue>] {
        fn queue(&self) -> &VecDeque<[<$building RecruitOrder>]> {
          &self.orders
        }

        fn queue_mut(&mut self) -> &mut VecDeque<[<$building RecruitOrder>]> {
          &mut self.orders
        }
      }

      #[must_use]
      #[derive(Clone, Debug, Deserialize, Serialize)]
      #[serde(rename_all = "camelCase")]
      pub struct [<$building RecruitOrder>] {
        id: [<$building RecruitOrderId>],
        squad: Squad,
        resources: Resources,
        workforce: Workforce,
        state: [<$building RecruitOrderState>],
      }

      impl [<$building RecruitOrder>] {
        #[inline]
        pub fn id(&self) -> [<$building RecruitOrderId>] {
          self.id
        }

        #[inline]
        pub fn squad(&self) -> &Squad {
          &self.squad
        }

        #[inline]
        pub fn resources(&self) -> &Resources {
          &self.resources
        }
      }

      impl From<[<$building RecruitOrder>]> for Squad {
        fn from(order: [<$building RecruitOrder>]) -> Self {
          order.squad
        }
      }

      impl InfrastructureQueueOrder for [<$building RecruitOrder>] {
        fn is_done(&self) -> bool {
          self.state.is_done()
        }

        fn set_done(&mut self) {
          self.state = [<$building RecruitOrderState>]::Done;
        }

        fn pending_workforce(&self) -> Option<Workforce> {
          self.state.pending_workforce()
        }

        fn pending_workforce_mut(&mut self) -> Option<&mut Workforce> {
          self.state.pending_workforce_mut()
        }
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
      pub struct [<$building RecruitOrderId>](Uuid);

      impl [<$building RecruitOrderId>] {
        #[must_use]
        pub fn new() -> Self {
          Self(Uuid::new_v4())
        }
      }

      impl Default for [<$building RecruitOrderId>] {
        fn default() -> Self {
          Self::new()
        }
      }

      #[derive(Clone, Debug, EnumIs, Deserialize, Serialize)]
      #[serde(tag = "kind", rename_all = "kebab-case")]
      pub enum [<$building RecruitOrderState>] {
        Pending { workforce: Workforce },
        Done,
      }

      impl [<$building RecruitOrderState>] {
        fn pending_workforce(&self) -> Option<Workforce> {
          if let Self::Pending { workforce } = self { Some(*workforce) } else { None }
        }

        fn pending_workforce_mut(&mut self) -> Option<&mut Workforce> {
          if let Self::Pending { workforce } = self { Some(workforce) } else { None }
        }
      }

      impl [<$building RecruitOrderState>] {
        fn new(workforce: Workforce) -> Self {
          Self::Pending { workforce }
        }
      }

      #[derive(Clone, Debug, Deserialize, Serialize)]
      #[serde(rename_all = "camelCase")]
      pub struct [<$building RecruitOrderRequest>] {
        pub coord: Coord,
        pub unit: [<$building UnitId>],
        pub chunks: NonZeroU32,
      }
    }
  };
}
