// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::{ContinentSize, Coord};
use crate::error::Error;
use crate::military::Military;
use crate::military::army::{Army, ArmyId, ArmyPersonnel, ArmyState, collapse_armies};
use crate::military::maneuver::{Maneuver, ManeuverId, ManeuverKind};
use crate::military::unit::stats::speed::Speed;
use crate::npc::bot::{Bot, BotId};
use crate::ruler::Ruler;
use std::assert_matches::assert_matches;

#[test]
fn spawn() {
  let size = ContinentSize::new(50);
  let mut military = Military::new(size);
  assert_eq!(military.count_armies(), 0);

  let coord = Coord::splat(0);
  let ruler = make_ruler("Bot 1");
  let personnel = ArmyPersonnel::splat(10);
  military.spawn(coord, ruler, personnel);

  assert_eq!(military.count_armies(), 1);
}

#[test]
fn collapse() {
  let ruler = make_ruler("Bot 1");
  let mut armies = Vec::with_capacity(15);

  for _ in 0..10 {
    armies.push(
      Army::builder()
        .owner(ruler.clone())
        .personnel(ArmyPersonnel::splat(10))
        .build(),
    );
  }

  armies.push(
    Army::builder()
      .owner(ruler.clone())
      .personnel(ArmyPersonnel::splat(10))
      .state(ArmyState::with_maneuver(ManeuverId::new()))
      .build(),
  );

  armies.push(
    Army::builder()
      .owner(make_ruler("Bot 2"))
      .personnel(ArmyPersonnel::splat(10))
      .build(),
  );

  armies.push(
    Army::builder()
      .owner(make_ruler("Bot 3"))
      .personnel(ArmyPersonnel::splat(10))
      .build(),
  );

  assert_eq!(armies.len(), 13);
  collapse_armies(&mut armies);
  assert_eq!(armies.len(), 4);
}

#[test]
fn intersection() {
  let size = ContinentSize::new(50);
  let mut military = Military::new(size);

  let ruler = make_ruler("Bot 1");
  let mut coords = Vec::with_capacity(10);

  let mut spawn = |coord: Coord| {
    let personnel = ArmyPersonnel::splat(10);
    military.spawn(coord, ruler.clone(), personnel);
  };

  for i in 0..10 {
    let coord = Coord::splat(i);
    spawn(coord);
    spawn(coord);
    spawn(coord);

    coords.push(coord);
  }

  assert_eq!(military.count_armies(), coords.len());

  let coords = coords.iter().take(3).copied();
  let military = military.intersection(coords).unwrap();
  assert_eq!(military.count_armies(), 3);
}

#[test]
fn origin_is_destination() {
  let coord = Coord::splat(0);
  let result = Maneuver::builder()
    .kind(ManeuverKind::Attack)
    .army(ArmyId::new())
    .origin(coord)
    .destination(coord)
    .speed(Speed::default())
    .build();

  assert_matches!(result, Err(Error::OriginIsDestination(..)));
}

#[test]
fn advance_done_maneuver() {
  let (_, mut maneuver) = Maneuver::builder()
    .kind(ManeuverKind::Attack)
    .army(ArmyId::new())
    .origin(Coord::splat(0))
    .destination(Coord::splat(1))
    .speed(Speed::new(5.0))
    .build()
    .unwrap();

  let result = try {
    for _ in 0..2 {
      maneuver.advance()?;
    }
  };

  assert_matches!(result, Err(Error::ManeuverIsDone(..)));
}

fn make_ruler(id: &str) -> Ruler {
  Ruler::from(&Bot::new(BotId::from(id)))
}
