// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::behavior::r#impl::idle::IdleBehavior;
use crate::behavior::score::BehaviorScore;
use crate::behavior::{Behavior, BehaviorProcessor};
use crate::error::Result;
use crate::resources::prelude::*;
use crate::ruler::Ruler;
use crate::world::World;
use bon::Builder;
use nil_num::mul_ceil::MulCeil;
use nil_util::iter::IterExt;
use nil_util::ops::TryExt;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{ControlFlow, Sub};
use strum::IntoEnumIterator;
use tap::{Conv, Pipe};

#[derive(Builder, Debug)]
pub struct TradeBehavior {
  ruler: Ruler,
}

impl TradeBehavior {
  pub const BUY_THRESHOLD: f64 = 0.1;
  pub const SELL_THRESHOLD: f64 = 0.9;
}

impl Behavior for TradeBehavior {
  fn score(&self, world: &World) -> Result<BehaviorScore> {
    if !self.ruler.is_bot() {
      return Ok(BehaviorScore::MIN);
    }

    let resources = world
      .ruler(&self.ruler)?
      .resources()
      .sum()
      .conv::<f64>();

    let capacity = world
      .get_storage_capacity(&self.ruler)?
      .mean();

    if capacity <= 0.0 {
      return Ok(BehaviorScore::MIN);
    }

    // The score increases as the resources approach the capacity, or when they approach zero.
    let score = (2.0 * (resources / capacity) - 1.0).powi(2);

    Ok(BehaviorScore::from(score))
  }

  fn behave(&self, world: &mut World) -> Result<ControlFlow<()>> {
    let mut behaviors = vec![IdleBehavior.boxed()];

    macro_rules! push {
      ($behavior:ident, $resource:ident) => {{
        let behavior = $behavior::builder()
          .ruler(self.ruler.clone())
          .resource(ResourceId::$resource)
          .marker(PhantomData::<$resource>)
          .build();

        behaviors.push(behavior.boxed());
      }};
    }

    let ruler_ref = world.ruler(&self.ruler)?;
    let resources = ruler_ref.resources();
    let gold = ruler_ref.gold();

    let vault = world.market().vault().resources();

    for id in ResourceId::iter() {
      let resource = resources.get(id);
      let amount = resource.as_f64();
      let capacity = world
        .get_storage_capacity_for(&self.ruler, id)?
        .conv::<f64>();

      let in_vault = vault.get(id).as_u32();

      if in_vault > 0 && gold > 0 && amount < capacity * Self::BUY_THRESHOLD {
        match id {
          ResourceId::Food => push!(BuyResourcesBehavior, Food),
          ResourceId::Iron => push!(BuyResourcesBehavior, Iron),
          ResourceId::Stone => push!(BuyResourcesBehavior, Stone),
          ResourceId::Wood => push!(BuyResourcesBehavior, Wood),
        }
      } else if amount > capacity * Self::SELL_THRESHOLD {
        match id {
          ResourceId::Food => push!(SellResourcesBehavior, Food),
          ResourceId::Iron => push!(SellResourcesBehavior, Iron),
          ResourceId::Stone => push!(SellResourcesBehavior, Stone),
          ResourceId::Wood => push!(SellResourcesBehavior, Wood),
        }
      }
    }

    BehaviorProcessor::new(world, behaviors).try_each()?;

    Ok(ControlFlow::Break(()))
  }
}

#[derive(Builder, Debug)]
pub struct BuyResourcesBehavior<T>
where
  T: Resource + Debug,
{
  ruler: Ruler,
  resource: ResourceId,
  marker: PhantomData<T>,
}

impl<T> BuyResourcesBehavior<T>
where
  T: Resource + Debug,
{
  fn threshold(&self, world: &World) -> Result<f64> {
    let capacity = world
      .get_storage_capacity_for(&self.ruler, self.resource)?
      .conv::<f64>();

    Ok(capacity.mul_ceil(TradeBehavior::BUY_THRESHOLD))
  }

  fn shortage(&self, world: &World) -> Result<f64> {
    let resource = world
      .ruler(&self.ruler)?
      .resources()
      .get(self.resource)
      .as_f64();

    Ok(self.threshold(world)?.sub(resource).max(0.0))
  }
}

impl<T> Behavior for BuyResourcesBehavior<T>
where
  T: Resource + Debug + 'static,
{
  fn score(&self, world: &World) -> Result<BehaviorScore> {
    let ruler_ref = world.ruler(&self.ruler)?;
    let market_price = ruler_ref
      .resources()
      .get(self.resource)
      .market_price();

    if ruler_ref.gold() < market_price {
      return Ok(BehaviorScore::MIN);
    }

    let threshold = self.threshold(world)?;
    let shortage = self.shortage(world)?;

    if threshold <= 0.0 || shortage <= 0.0 {
      return Ok(BehaviorScore::MIN);
    }

    Ok(BehaviorScore::new(shortage / threshold))
  }

  fn behave(&self, world: &mut World) -> Result<ControlFlow<()>> {
    let ruler_ref = world.ruler(&self.ruler)?;
    let gold = ruler_ref.gold();

    macro_rules! buyable_amount {
      ($resource:ident) => {{
        world
          .market()
          .buyable_amount($resource::MARKET_PRICE, gold)
          .pipe($resource::new)
          .as_resources()
      }};
    }

    let mut buy_amount = match self.resource {
      ResourceId::Food => buyable_amount!(Food),
      ResourceId::Iron => buyable_amount!(Iron),
      ResourceId::Stone => buyable_amount!(Stone),
      ResourceId::Wood => buyable_amount!(Wood),
    };

    let shortage = self.shortage(world)?.floor();
    match self.resource {
      ResourceId::Food => {
        Resource::clamp(&mut buy_amount.food, 0, *Food::from(shortage));
      }
      ResourceId::Iron => {
        Resource::clamp(&mut buy_amount.iron, 0, *Iron::from(shortage));
      }
      ResourceId::Stone => {
        Resource::clamp(&mut buy_amount.stone, 0, *Stone::from(shortage));
      }
      ResourceId::Wood => {
        Resource::clamp(&mut buy_amount.wood, 0, *Wood::from(shortage));
      }
    }

    let vault_resources = world.market().vault().resources();

    buy_amount.iter_mut().for_each(|resource| {
      let in_vault = vault_resources.get(resource.id());
      Resource::clamp(resource, 0, in_vault.as_u32());
    });

    if !buy_amount.is_empty() {
      world.buy_resources_with_emit(&self.ruler, buy_amount, false)?;
    }

    Ok(ControlFlow::Break(()))
  }
}

#[derive(Builder, Debug)]
pub struct SellResourcesBehavior<T>
where
  T: Resource + Debug,
{
  ruler: Ruler,
  resource: ResourceId,
  marker: PhantomData<T>,
}

impl<T> SellResourcesBehavior<T>
where
  T: Resource + Debug,
{
  fn threshold(&self, world: &World) -> Result<f64> {
    let capacity = world
      .get_storage_capacity_for(&self.ruler, self.resource)?
      .conv::<f64>();

    Ok(capacity.mul_ceil(TradeBehavior::SELL_THRESHOLD))
  }

  fn surplus(&self, world: &World, threshold: Option<f64>) -> Result<f64> {
    let resource = world
      .ruler(&self.ruler)?
      .resources()
      .get(self.resource)
      .as_f64();

    let threshold = threshold.unwrap_or_try_else(|| self.threshold(world))?;

    Ok(resource.sub(threshold).max(0.0))
  }
}

impl<T> Behavior for SellResourcesBehavior<T>
where
  T: Resource + Debug + 'static,
{
  fn score(&self, world: &World) -> Result<BehaviorScore> {
    let threshold = self.threshold(world)?;
    let surplus = self.surplus(world, Some(threshold))?;

    if threshold <= 0.0 || surplus <= 0.0 {
      return Ok(BehaviorScore::MIN);
    }

    Ok(BehaviorScore::new(surplus / threshold))
  }

  fn behave(&self, world: &mut World) -> Result<ControlFlow<()>> {
    let surplus = self.surplus(world, None)?.floor();
    if surplus < 1.0 {
      return Ok(ControlFlow::Break(()));
    }

    let sell_amount = match self.resource {
      ResourceId::Food => Food::from(surplus).as_resources(),
      ResourceId::Iron => Iron::from(surplus).as_resources(),
      ResourceId::Stone => Stone::from(surplus).as_resources(),
      ResourceId::Wood => Wood::from(surplus).as_resources(),
    };

    if !sell_amount.is_empty() {
      world.sell_resources_with_emit(&self.ruler, sell_amount, false)?;
    }

    Ok(ControlFlow::Break(()))
  }
}
