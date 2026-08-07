// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::player::PlayerId;
use crate::resources::Resources;
use crate::resources::gold::Gold;
use crate::ruler::Ruler;
use crate::tests::{make_world, spawn_player};
use std::assert_matches;

#[test]
fn buy_resources() -> Result<()> {
  let mut world = make_world()?;
  let player = PlayerId::from("Player A");
  spawn_player(&mut world, player.as_str())?;

  let ruler = Ruler::from(player);
  let initial_resources = Resources::splat(1000);
  world
    .ruler_mut(&ruler)?
    .resources_mut()
    .replace(initial_resources);

  let initial_gold = Gold::new(10_000);
  *world.ruler_mut(&ruler)?.gold_mut() = initial_gold;

  let resources_to_buy = Resources::splat(1000);
  let fee = resources_to_buy * world.market().fee();
  let cost = Gold::from(resources_to_buy + fee);

  world
    .market_mut()
    .vault_mut()
    .set(resources_to_buy);

  world.buy_resources(&ruler, resources_to_buy)?;

  assert_eq!(
    world.ruler(&ruler)?.resources(),
    initial_resources + resources_to_buy
  );

  assert_eq!(world.ruler(&ruler)?.gold(), initial_gold - cost);
  assert_eq!(world.market().vault().resources(), Resources::splat(0));

  Ok(())
}

#[test]
fn sell_resources() -> Result<()> {
  let mut world = make_world()?;
  let player = PlayerId::from("Player A");
  spawn_player(&mut world, player.as_str())?;

  let ruler = Ruler::from(player);
  let initial_resources = Resources::splat(5000);
  world
    .ruler_mut(&ruler)?
    .resources_mut()
    .replace(initial_resources);

  let resources_to_sell = Resources::splat(1000);
  world.sell_resources(&ruler, resources_to_sell)?;

  assert_eq!(
    world.ruler(&ruler)?.resources(),
    initial_resources
      .checked_sub(resources_to_sell)
      .ok_or(Error::InsufficientResources)?
  );

  assert_eq!(world.market().vault().resources(), resources_to_sell);
  assert_eq!(world.ruler(&ruler)?.gold(), Gold::from(resources_to_sell));

  Ok(())
}

#[test]
fn sell_then_buy_resources() -> Result<()> {
  let mut world = make_world()?;
  let seller = PlayerId::from("Seller");
  spawn_player(&mut world, seller.as_str())?;

  let buyer = PlayerId::from("Buyer");
  spawn_player(&mut world, buyer.as_str())?;

  let seller = Ruler::from(seller);
  let seller_initial_resources = Resources::splat(5000);
  let seller_initial_gold = Gold::new(1000);

  let buyer = Ruler::from(buyer);
  let buyer_initial_resources = Resources::splat(1000);
  let buyer_initial_gold = Gold::new(10_000);

  let resources_to_trade = Resources::splat(1000);
  let fee = resources_to_trade * world.market().fee();
  let cost = Gold::from(resources_to_trade + fee);

  world
    .ruler_mut(&seller)?
    .resources_mut()
    .replace(seller_initial_resources);

  *world.ruler_mut(&seller)?.gold_mut() = seller_initial_gold;

  world
    .ruler_mut(&buyer)?
    .resources_mut()
    .replace(buyer_initial_resources);

  *world.ruler_mut(&buyer)?.gold_mut() = buyer_initial_gold;

  world.sell_resources(&seller, resources_to_trade)?;
  world.buy_resources(&buyer, resources_to_trade)?;

  assert_eq!(
    world.ruler(&seller)?.resources(),
    seller_initial_resources
      .checked_sub(resources_to_trade)
      .ok_or(Error::InsufficientResources)?
  );

  assert_eq!(
    world.ruler(&seller)?.gold(),
    seller_initial_gold + resources_to_trade
  );

  assert_eq!(
    world.ruler(&buyer)?.resources(),
    buyer_initial_resources + resources_to_trade
  );

  assert_eq!(world.ruler(&buyer)?.gold(), buyer_initial_gold - cost);
  assert_eq!(world.market().vault().resources(), Resources::splat(0));

  Ok(())
}

#[test]
fn send_resources() -> Result<()> {
  let mut world = make_world()?;
  let player_a = PlayerId::from("Player A");
  spawn_player(&mut world, player_a.as_str())?;

  let player_b = PlayerId::from("Player B");
  spawn_player(&mut world, player_b.as_str())?;

  let ruler_a = Ruler::from(player_a);
  let ruler_b = Ruler::from(player_b);
  let initial_resources_a = Resources::splat(5000);
  let initial_resources_b = Resources::splat(1000);

  world
    .ruler_mut(&ruler_a)?
    .resources_mut()
    .replace(initial_resources_a);

  world
    .ruler_mut(&ruler_b)?
    .resources_mut()
    .replace(initial_resources_b);

  let resources_to_send = Resources::splat(1000);
  world.send_resources(&ruler_a, &ruler_b, resources_to_send)?;

  let remaining_resources_a = world.ruler(&ruler_a)?.resources();
  let fee = resources_to_send * world.market().fee();

  assert_eq!(
    remaining_resources_a,
    initial_resources_a
      .checked_sub(resources_to_send + fee)
      .ok_or(Error::InsufficientResources)?
  );

  let new_resources_b = world.ruler(&ruler_b)?.resources();
  assert_eq!(new_resources_b, initial_resources_b + resources_to_send);

  Ok(())
}

#[test]
fn fee_is_stored_in_vault() -> Result<()> {
  let mut world = make_world()?;
  let player_a = PlayerId::from("Player A");
  spawn_player(&mut world, player_a.as_str())?;

  let player_b = PlayerId::from("Player B");
  spawn_player(&mut world, player_b.as_str())?;

  let ruler_a = Ruler::from(player_a);
  let ruler_b = Ruler::from(player_b);
  let initial_resources_a = Resources::splat(5000);
  let initial_resources_b = Resources::splat(1000);

  world
    .ruler_mut(&ruler_a)?
    .resources_mut()
    .replace(initial_resources_a);

  world
    .ruler_mut(&ruler_b)?
    .resources_mut()
    .replace(initial_resources_b);

  let resources_to_send = Resources::splat(1000);
  let fee = resources_to_send * world.market().fee();
  world.send_resources(&ruler_a, &ruler_b, resources_to_send)?;

  let market_vault = world.market().vault();
  assert_eq!(market_vault.resources(), fee);

  Ok(())
}

#[test]
fn cannot_send_resources_to_self() -> Result<()> {
  let mut world = make_world()?;
  let player = PlayerId::from("Player A");
  spawn_player(&mut world, player.as_str())?;

  let ruler = Ruler::from(player);
  let resources = Resources::splat(1000);

  let result = world.send_resources(&ruler, &ruler, resources);
  assert_matches!(result, Err(Error::ResourceReceiverIsSender(..)));

  Ok(())
}

#[test]
fn cannot_send_resources_if_insufficient() -> Result<()> {
  let mut world = make_world()?;
  let player_a = PlayerId::from("Player A");
  spawn_player(&mut world, player_a.as_str())?;

  let player_b = PlayerId::from("Player B");
  spawn_player(&mut world, player_b.as_str())?;

  let ruler_a = Ruler::from(player_a);
  let ruler_b = Ruler::from(player_b);
  let resources = Resources::splat(1_000_000);

  let result = world.send_resources(&ruler_a, &ruler_b, resources);
  assert_matches!(result, Err(Error::InsufficientResources));

  Ok(())
}
