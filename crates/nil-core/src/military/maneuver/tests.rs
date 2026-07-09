// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::coord::Coord;
use crate::error::Error;
use crate::military::army::ArmyId;
use crate::military::maneuver::distance::ManeuverDistance;
use crate::military::maneuver::{Maneuver, ManeuverDirection, ManeuverKind};
use crate::military::unit::stats::speed::Speed;
use std::assert_matches;

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

#[test]
fn cancel_maneuver() {
  let (_, mut maneuver) = Maneuver::builder()
    .kind(ManeuverKind::Attack)
    .army(ArmyId::new())
    .origin(Coord::splat(0))
    .destination(Coord::splat(1))
    .speed(Speed::new(5.0))
    .build()
    .unwrap();

  assert_matches!(maneuver.direction(), ManeuverDirection::Going);

  maneuver.cancel().unwrap();

  assert_matches!(maneuver.direction(), ManeuverDirection::Returning);
  assert_eq!(
    maneuver.pending_distance(),
    Some(ManeuverDistance::from(1.0f64))
  );

  assert_matches!(maneuver.cancel(), Err(Error::ManeuverIsReturning(..)));
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
