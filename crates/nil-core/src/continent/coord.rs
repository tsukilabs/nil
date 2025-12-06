// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::ContinentSize;
use crate::error::Result;
use derive_more::{From, Into};
use glam::u8::U8Vec2;
use itertools::Itertools;
use serde::de::{self, Error as _, MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coord(U8Vec2);

impl Coord {
  #[inline]
  #[must_use]
  pub const fn new(x: u8, y: u8) -> Self {
    Self(U8Vec2::new(x, y))
  }

  #[inline]
  #[must_use]
  pub const fn splat(value: u8) -> Self {
    Self(U8Vec2::splat(value))
  }

  #[inline]
  pub const fn x(&self) -> u8 {
    self.0.x
  }

  #[inline]
  pub const fn y(&self) -> u8 {
    self.0.y
  }

  #[inline]
  pub fn distance(&self, rhs: Coord) -> Distance {
    Distance::new(self.0.chebyshev_distance(rhs.0))
  }

  #[inline]
  pub fn is_within_continent(&self, size: ContinentSize) -> bool {
    size > self.x() && size > self.y()
  }

  pub fn is_within_distance(&self, other: Coord, distance: Distance) -> bool {
    let x0 = i16::from(self.x());
    let y0 = i16::from(self.y());
    let distance = i16::from(distance);

    let absx = (i16::from(other.x()) - x0).abs();
    let absy = (i16::from(other.y()) - y0).abs();
    absx.max(absy) <= distance
  }

  #[inline]
  #[must_use]
  pub fn within_distance(self, distance: Distance) -> Vec<Self> {
    within_distance(self, distance, false)
  }

  #[inline]
  #[must_use]
  pub fn within_distance_inclusive(self, distance: Distance) -> Vec<Self> {
    within_distance(self, distance, true)
  }
}

fn within_distance(origin: Coord, distance: Distance, inclusive: bool) -> Vec<Coord> {
  let mut coords = Vec::new();
  let x0 = i16::from(origin.x());
  let y0 = i16::from(origin.y());
  let distance = i16::from(distance);

  for x in (x0 - distance)..=(x0 + distance) {
    for y in (y0 - distance)..=(y0 + distance) {
      let absx = (x - x0).abs();
      let absy = (y - y0).abs();
      if absx.max(absy) <= distance
        && (inclusive || x != x0 || y != y0)
        && let Ok(x) = u8::try_from(x)
        && let Ok(y) = u8::try_from(y)
      {
        coords.push(Coord::new(x, y));
      }
    }
  }

  coords.into_iter().unique().collect()
}

impl Add for Coord {
  type Output = Coord;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_add(rhs.0))
  }
}

impl Sub for Coord {
  type Output = Coord;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_sub(rhs.0))
  }
}

impl From<(u8, u8)> for Coord {
  fn from((x, y): (u8, u8)) -> Self {
    Self::new(x, y)
  }
}

impl fmt::Display for Coord {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:03}|{:03}", self.0.x, self.0.y)
  }
}

impl Serialize for Coord {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut coord = serializer.serialize_struct("Coord", 2)?;
    coord.serialize_field("x", &self.0.x)?;
    coord.serialize_field("y", &self.0.y)?;
    coord.end()
  }
}

impl<'de> Deserialize<'de> for Coord {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_struct("Coord", &CoordVisitor::FIELD, CoordVisitor)
  }
}

struct CoordVisitor;

impl CoordVisitor {
  const FIELD: [&str; 2] = ["x", "y"];
}

impl<'de> Visitor<'de> for CoordVisitor {
  type Value = Coord;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("struct Coord")
  }

  fn visit_seq<V>(self, mut seq: V) -> Result<Coord, V::Error>
  where
    V: SeqAccess<'de>,
  {
    let x = seq
      .next_element()?
      .ok_or_else(|| de::Error::invalid_length(0, &self))?;

    let y = seq
      .next_element()?
      .ok_or_else(|| de::Error::invalid_length(1, &self))?;

    Ok(Coord::new(x, y))
  }

  fn visit_map<V>(self, mut map: V) -> Result<Coord, V::Error>
  where
    V: MapAccess<'de>,
  {
    let mut x = None;
    let mut y = None;

    while let Some(key) = map.next_key::<Cow<'static, str>>()? {
      match key.as_ref() {
        "x" => x = Some(map.next_value()?),
        "y" => y = Some(map.next_value()?),
        _ => {}
      }
    }

    Ok(Coord::new(
      x.ok_or_else(|| V::Error::missing_field("x"))?,
      y.ok_or_else(|| V::Error::missing_field("y"))?,
    ))
  }
}

#[derive(
  Clone, Copy, Debug, Default, From, Into, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
#[into(i16, u8, f64)]
pub struct Distance(u8);

impl Distance {
  #[inline]
  pub const fn new(distance: u8) -> Self {
    Self(distance)
  }
}

impl PartialEq<u8> for Distance {
  fn eq(&self, other: &u8) -> bool {
    self.0.eq(other)
  }
}

impl Add for Distance {
  type Output = Distance;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_add(rhs.0))
  }
}

impl AddAssign for Distance {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl Sub for Distance {
  type Output = Distance;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_sub(rhs.0))
  }
}

impl SubAssign for Distance {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl Add<u8> for Distance {
  type Output = Distance;

  fn add(self, rhs: u8) -> Self::Output {
    Self(self.0.saturating_add(rhs))
  }
}

impl AddAssign<u8> for Distance {
  fn add_assign(&mut self, rhs: u8) {
    *self = *self + rhs;
  }
}

impl Sub<u8> for Distance {
  type Output = Distance;

  fn sub(self, rhs: u8) -> Self::Output {
    Self(self.0.saturating_sub(rhs))
  }
}

impl SubAssign<u8> for Distance {
  fn sub_assign(&mut self, rhs: u8) {
    *self = *self - rhs;
  }
}
