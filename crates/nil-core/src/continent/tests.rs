// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::{Continent, ContinentSize, Coord};

#[test]
fn field() {
  each_coord(100, |continent, coord| {
    assert!(continent.field(coord).is_ok());
  });
}

#[test]
fn center() {
  let mut continent = Continent::new(100);
  assert_eq!(continent.center(), Coord::splat(50));

  continent = Continent::new(200);
  assert_eq!(continent.center(), Coord::splat(100));
}

#[test]
fn coord_splat() {
  let coord = Coord::splat(100);
  assert_eq!(coord.x(), coord.y());
  assert_eq!(coord, Coord::new(100, 100));
}

#[test]
fn is_within_continent() {
  let size = ContinentSize::new(100);
  assert!(Coord::splat(0).is_within_continent(size));
  assert!(Coord::splat(25).is_within_continent(size));
  assert!(Coord::splat(50).is_within_continent(size));
  assert!(Coord::splat(99).is_within_continent(size));
  assert!(!Coord::splat(100).is_within_continent(size));
}

fn each_coord(size: u8, f: impl Fn(&mut Continent, Coord)) {
  assert!(size >= ContinentSize::MIN.get());
  let mut continent = Continent::new(size);
  (0..size).into_iter().for_each(|x| {
    (0..size).into_iter().for_each(|y| {
      f(&mut continent, Coord::new(x, y));
    });
  })
}
