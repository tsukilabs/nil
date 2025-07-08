// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Continent;
use crate::village::Coord;

#[test]
fn cell() {
  each_coord(|continent, coord| {
    assert!(continent.field(coord).is_ok());
  });
}

#[test]
fn index_to_coord() {
  each_coord(|continent, coord| {
    let index = continent.index(coord);
    assert_eq!(coord, continent.coord(index).unwrap());
  });
}

#[test]
fn default_continent_is_empty() {
  each_coord(|continent, coord| {
    let field = continent.field(coord).unwrap();
    assert!(field.is_empty());
  });
}

fn each_coord(f: impl Fn(&mut Continent, Coord)) {
  let mut continent = Continent::default();
  (0..100).into_iter().for_each(|x| {
    (0..100).into_iter().for_each(|y| {
      f(&mut continent, Coord::new(x, y));
    });
  })
}
