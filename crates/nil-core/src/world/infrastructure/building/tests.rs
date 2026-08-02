// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::infrastructure::building::BuildingId;
use crate::infrastructure::building::r#impl::prefecture::build_queue::{
  PrefectureBuildOrderKind,
  PrefectureBuildOrderRequest,
};
use crate::lv;
use crate::player::PlayerId;
use crate::resources::Resources;
use crate::ruler::Ruler;
use crate::tests::{
  INFRASTRUCTURE_STATS,
  get_first_bot,
  get_first_coord,
  get_first_precursor,
  make_world,
  spawn_player,
};

#[test]
fn prefecture_build_order_deducts_player_resources() -> Result<()> {
  let mut world = make_world()?;
  let player = PlayerId::from("Player A");
  spawn_player(&mut world, player.as_str())?;

  let ruler = Ruler::from(player);
  let initial_resources = Resources::splat(5000);
  world
    .ruler_mut(&ruler)?
    .resources_mut()
    .set(initial_resources);

  let coord = get_first_coord(&world, ruler.clone());
  let request = PrefectureBuildOrderRequest {
    coord,
    building: BuildingId::Prefecture,
    kind: PrefectureBuildOrderKind::Construction,
  };

  world.add_prefecture_build_order(&request)?;

  let remaining_resources = world.ruler(&ruler)?.resources();
  let required_resources = INFRASTRUCTURE_STATS
    .building(BuildingId::Prefecture)?
    .get(lv!(2))?
    .resources;

  assert_eq!(
    remaining_resources,
    initial_resources
      .checked_sub(required_resources)
      .ok_or(Error::InsufficientResources)?
  );

  Ok(())
}

#[test]
fn cancel_prefecture_build_order_refunds_player_resources() -> Result<()> {
  let mut world = make_world()?;
  let player = PlayerId::from("Player A");
  spawn_player(&mut world, player.as_str())?;

  let ruler = Ruler::from(player);
  let initial_resources = Resources::splat(5000);
  world
    .ruler_mut(&ruler)?
    .resources_mut()
    .set(initial_resources);

  let coord = get_first_coord(&world, ruler.clone());
  let request = PrefectureBuildOrderRequest {
    coord,
    building: BuildingId::Prefecture,
    kind: PrefectureBuildOrderKind::Construction,
  };

  world.add_prefecture_build_order(&request)?;
  world.cancel_prefecture_build_order(coord)?;

  let remaining_resources = world.ruler(&ruler)?.resources();
  assert_eq!(remaining_resources, initial_resources);

  Ok(())
}

#[test]
fn prefecture_build_order_deducts_bot_resources() -> Result<()> {
  let mut world = make_world()?;
  let bot = get_first_bot(&world);

  let ruler = Ruler::from(bot);
  let initial_resources = Resources::splat(5000);
  world
    .ruler_mut(&ruler)?
    .resources_mut()
    .set(initial_resources);

  let coord = get_first_coord(&world, ruler.clone());
  world
    .city_mut(coord)?
    .infrastructure_mut()
    .building_mut(BuildingId::Prefecture)
    .set_level(lv!(1));

  let request = PrefectureBuildOrderRequest {
    coord,
    building: BuildingId::Prefecture,
    kind: PrefectureBuildOrderKind::Construction,
  };

  world.add_prefecture_build_order(&request)?;

  let remaining_resources = world.ruler(&ruler)?.resources();
  let required_resources = INFRASTRUCTURE_STATS
    .building(BuildingId::Prefecture)?
    .get(lv!(2))?
    .resources;

  assert_eq!(
    remaining_resources,
    initial_resources
      .checked_sub(required_resources)
      .ok_or(Error::InsufficientResources)?
  );

  Ok(())
}

#[test]
fn cancel_prefecture_build_order_refunds_bot_resources() -> Result<()> {
  let mut world = make_world()?;
  let bot = get_first_bot(&world);

  let ruler = Ruler::from(bot);
  let initial_resources = Resources::splat(5000);
  world
    .ruler_mut(&ruler)?
    .resources_mut()
    .set(initial_resources);

  let coord = get_first_coord(&world, ruler.clone());
  world
    .city_mut(coord)?
    .infrastructure_mut()
    .building_mut(BuildingId::Prefecture)
    .set_level(lv!(1));

  let request = PrefectureBuildOrderRequest {
    coord,
    building: BuildingId::Prefecture,
    kind: PrefectureBuildOrderKind::Construction,
  };

  world.add_prefecture_build_order(&request)?;
  world.cancel_prefecture_build_order(coord)?;

  let remaining_resources = world.ruler(&ruler)?.resources();
  assert_eq!(remaining_resources, initial_resources);

  Ok(())
}

#[test]
fn prefecture_build_order_deducts_precursor_resources() -> Result<()> {
  let mut world = make_world()?;
  let precursor = get_first_precursor(&world);

  let ruler = Ruler::from(precursor);
  let initial_resources = Resources::splat(5000);
  world
    .ruler_mut(&ruler)?
    .resources_mut()
    .set(initial_resources);

  let coord = get_first_coord(&world, ruler.clone());
  world
    .city_mut(coord)?
    .infrastructure_mut()
    .building_mut(BuildingId::Prefecture)
    .set_level(lv!(1));

  let request = PrefectureBuildOrderRequest {
    coord,
    building: BuildingId::Prefecture,
    kind: PrefectureBuildOrderKind::Construction,
  };

  world.add_prefecture_build_order(&request)?;

  let remaining_resources = world.ruler(&ruler)?.resources();
  let required_resources = INFRASTRUCTURE_STATS
    .building(BuildingId::Prefecture)?
    .get(lv!(2))?
    .resources;

  assert_eq!(
    remaining_resources,
    initial_resources
      .checked_sub(required_resources)
      .ok_or(Error::InsufficientResources)?
  );

  Ok(())
}

#[test]
fn cancel_prefecture_build_order_refunds_precursor_resources() -> Result<()> {
  let mut world = make_world()?;
  let precursor = get_first_precursor(&world);

  let ruler = Ruler::from(precursor);
  let initial_resources = Resources::splat(5000);
  world
    .ruler_mut(&ruler)?
    .resources_mut()
    .set(initial_resources);

  let coord = get_first_coord(&world, ruler.clone());
  world
    .city_mut(coord)?
    .infrastructure_mut()
    .building_mut(BuildingId::Prefecture)
    .set_level(lv!(1));

  let request = PrefectureBuildOrderRequest {
    coord,
    building: BuildingId::Prefecture,
    kind: PrefectureBuildOrderKind::Construction,
  };

  world.add_prefecture_build_order(&request)?;
  world.cancel_prefecture_build_order(coord)?;

  let remaining_resources = world.ruler(&ruler)?.resources();
  assert_eq!(remaining_resources, initial_resources);

  Ok(())
}
