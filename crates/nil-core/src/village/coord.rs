// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use glam::u8::U8Vec2;
use serde::de::{self, Error as _, MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt;

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
  pub fn distance(&self, rhs: Coord) -> u8 {
    self.0.chebyshev_distance(rhs.0)
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

    // Lua wouldn't be able to deserialize it if we used a plain `&str` here.
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
