// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::coord::Coord;
use crate::error::{Error, Result};
use crate::infrastructure::building::r#impl::stable::recruit_queue::{
  StableRecruitOrderId,
  StableRecruitOrderRequest,
};
use crate::infrastructure::queue::InfrastructureQueue;
use crate::military::unit::r#impl::light_cavalry::LightCavalry;
use crate::military::unit::{StableUnitId, UnitId};
use crate::player::PlayerId;
use crate::resources::Resources;
use crate::ruler::Ruler;
use crate::tests::{get_first_bot, get_first_coord, get_first_precursor, make_world, spawn_player};
use crate::world::World;
use std::num::NonZeroU32;
use tap::Pipe;

#[test]
fn stable_recruit_order_deducts_player_resources() -> Result<()> {
  let mut world = make_world()?;
  let player = PlayerId::from("Player A");
  spawn_player(&mut world, player.as_str())?;

  let ruler = Ruler::from(player);
  let initial_resources = Resources::splat(5000);
  world
    .ruler_mut(&ruler)?
    .resources_mut()
    .replace(initial_resources);

  let coord = get_first_coord(&world, ruler.clone());

  world.add_stable_recruit_order(&req(coord))?;

  let remaining_resources = world.ruler(&ruler)?.resources();
  let required_resources = LightCavalry::CHUNK.resources();

  assert_eq!(
    remaining_resources,
    initial_resources
      .checked_sub(required_resources)
      .ok_or(Error::InsufficientResources)?
  );

  Ok(())
}

#[test]
fn cancel_stable_recruit_order_refunds_player_resources() -> Result<()> {
  let mut world = make_world()?;
  let player = PlayerId::from("Player A");
  spawn_player(&mut world, player.as_str())?;

  let ruler = Ruler::from(player);
  let initial_resources = Resources::splat(5000);
  world
    .ruler_mut(&ruler)?
    .resources_mut()
    .replace(initial_resources);

  let coord = get_first_coord(&world, ruler.clone());

  world.add_stable_recruit_order(&req(coord))?;
  world.cancel_stable_recruit_order(coord, order_id(&world, coord)?)?;

  let remaining_resources = world.ruler(&ruler)?.resources();
  assert_eq!(remaining_resources, initial_resources);

  Ok(())
}

#[test]
fn stable_recruit_order_deducts_bot_resources() -> Result<()> {
  let mut world = make_world()?;
  let bot = get_first_bot(&world);

  let ruler = Ruler::from(bot);
  let initial_resources = Resources::splat(5000);
  world
    .ruler_mut(&ruler)?
    .resources_mut()
    .replace(initial_resources);

  let coord = get_first_coord(&world, ruler.clone());

  world.add_stable_recruit_order(&req(coord))?;

  let remaining_resources = world.ruler(&ruler)?.resources();
  let required_resources = LightCavalry::CHUNK.resources();

  assert_eq!(
    remaining_resources,
    initial_resources
      .checked_sub(required_resources)
      .ok_or(Error::InsufficientResources)?
  );

  Ok(())
}

#[test]
fn cancel_stable_recruit_order_refunds_bot_resources() -> Result<()> {
  let mut world = make_world()?;
  let bot = get_first_bot(&world);

  let ruler = Ruler::from(bot);
  let initial_resources = Resources::splat(5000);
  world
    .ruler_mut(&ruler)?
    .resources_mut()
    .replace(initial_resources);

  let coord = get_first_coord(&world, ruler.clone());
  world.add_stable_recruit_order(&req(coord))?;
  world.cancel_stable_recruit_order(coord, order_id(&world, coord)?)?;

  let remaining_resources = world.ruler(&ruler)?.resources();
  assert_eq!(remaining_resources, initial_resources);

  Ok(())
}

#[test]
fn stable_recruit_order_deducts_precursor_resources() -> Result<()> {
  let mut world = make_world()?;
  let precursor = get_first_precursor(&world);

  let ruler = Ruler::from(precursor);
  let initial_resources = Resources::splat(5000);
  world
    .ruler_mut(&ruler)?
    .resources_mut()
    .replace(initial_resources);

  let coord = get_first_coord(&world, ruler.clone());

  world.add_stable_recruit_order(&req(coord))?;

  let remaining_resources = world.ruler(&ruler)?.resources();
  let required_resources = LightCavalry::CHUNK.resources();

  assert_eq!(
    remaining_resources,
    initial_resources
      .checked_sub(required_resources)
      .ok_or(Error::InsufficientResources)?
  );

  Ok(())
}

#[test]
fn cancel_stable_recruit_order_refunds_precursor_resources() -> Result<()> {
  let mut world = make_world()?;
  let precursor = get_first_precursor(&world);

  let ruler = Ruler::from(precursor);
  let initial_resources = Resources::splat(5000);
  world
    .ruler_mut(&ruler)?
    .resources_mut()
    .replace(initial_resources);

  let coord = get_first_coord(&world, ruler.clone());
  world.add_stable_recruit_order(&req(coord))?;
  world.cancel_stable_recruit_order(coord, order_id(&world, coord)?)?;

  let remaining_resources = world.ruler(&ruler)?.resources();
  assert_eq!(remaining_resources, initial_resources);

  Ok(())
}

fn req(coord: Coord) -> StableRecruitOrderRequest {
  StableRecruitOrderRequest {
    coord,
    unit: StableUnitId::LightCavalry,
    chunks: NonZeroU32::MIN,
  }
}

fn order_id(world: &World, coord: Coord) -> Result<StableRecruitOrderId> {
  let queue = world
    .city(coord)?
    .infrastructure()
    .stable()
    .recruit_queue()
    .queue();

  assert_eq!(queue.len(), 1);

  queue
    .iter()
    .find(|order| order.squad().id() == UnitId::LightCavalry)
    .unwrap()
    .id()
    .pipe(Ok)
}
