// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Continent;
use crate::village::Coord;

#[test]
fn field() {
  each_coord(100, |continent, coord| {
    assert!(continent.field(coord).is_ok());
  });
}

#[test]
fn default_continent_is_empty() {
  each_coord(100, |continent, coord| {
    let field = continent.field(coord).unwrap();
    assert!(field.is_empty());
  });
}

#[test]
fn center() {
  let mut continent = Continent::new(50);
  assert_eq!(continent.center(), Coord::splat(25));

  continent = Continent::new(100);
  assert_eq!(continent.center(), Coord::splat(50));

  continent = Continent::new(200);
  assert_eq!(continent.center(), Coord::splat(100));
}

fn each_coord(size: u8, f: impl Fn(&mut Continent, Coord)) {
  let mut continent = Continent::new(size);
  (0..100).into_iter().for_each(|x| {
    (0..100).into_iter().for_each(|y| {
      f(&mut continent, Coord::new(x, y));
    });
  })
}
