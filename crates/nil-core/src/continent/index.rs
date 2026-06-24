// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::{ContinentSize, Coord};
use crate::error::{Error, Result};
use derive_more::Display;
use nil_util::ConstDeref;
use serde::{Deserialize, Serialize, Serializer, ser};
use std::ops::{Div, Rem};

#[derive(Copy, Debug, Display, Hash, Deserialize, ConstDeref)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(as = "u32"))]
pub struct ContinentIndex(usize);

impl ContinentIndex {
  #[inline]
  pub const fn new(index: usize) -> Self {
    Self(index)
  }

  pub fn from_coord(coord: Coord, size: ContinentSize) -> Self {
    let size = usize::from(size);
    let x = usize::from(coord.x());
    let y = usize::from(coord.y());
    let index = (y * size) + x;

    debug_assert!(x < size);
    debug_assert!(y < size);

    ContinentIndex(index)
  }

  pub fn to_coord(self, size: ContinentSize) -> Result<Coord> {
    let x = self % size;
    let y = self / size;

    debug_assert!(x < size);
    debug_assert!(y < size);

    Ok(Coord::new(
      u8::try_from(x).map_err(|_| Error::IndexOutOfBounds(self))?,
      u8::try_from(y).map_err(|_| Error::IndexOutOfBounds(self))?,
    ))
  }
}

impl Serialize for ContinentIndex {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    u32::try_from(self.0)
      .map_err(|_| ser::Error::custom("index out of bounds"))?
      .serialize(serializer)
  }
}

const impl From<usize> for ContinentIndex {
  fn from(value: usize) -> Self {
    Self::new(value)
  }
}

const impl From<ContinentIndex> for usize {
  fn from(value: ContinentIndex) -> Self {
    value.0
  }
}

const impl Div<usize> for ContinentIndex {
  type Output = usize;

  fn div(self, rhs: usize) -> Self::Output {
    self.0 / rhs
  }
}

const impl Div<ContinentSize> for ContinentIndex {
  type Output = usize;

  fn div(self, rhs: ContinentSize) -> Self::Output {
    self.0 / usize::from(rhs)
  }
}

const impl Rem<usize> for ContinentIndex {
  type Output = usize;

  fn rem(self, rhs: usize) -> Self::Output {
    self.0 % rhs
  }
}

const impl Rem<ContinentSize> for ContinentIndex {
  type Output = usize;

  fn rem(self, rhs: ContinentSize) -> Self::Output {
    self.0 % usize::from(rhs)
  }
}

pub trait ContinentKey {
  fn into_index(self, size: ContinentSize) -> ContinentIndex;

  fn into_coord(self, size: ContinentSize) -> Result<Coord>
  where
    Self: Sized,
  {
    self.into_index(size).to_coord(size)
  }
}

impl ContinentKey for ContinentIndex {
  fn into_index(self, _: ContinentSize) -> ContinentIndex {
    self
  }
}

impl ContinentKey for Coord {
  fn into_index(self, size: ContinentSize) -> ContinentIndex {
    ContinentIndex::from_coord(self, size)
  }

  fn into_coord(self, _: ContinentSize) -> Result<Coord>
  where
    Self: Sized,
  {
    Ok(self)
  }
}

impl ContinentKey for usize {
  fn into_index(self, _: ContinentSize) -> ContinentIndex {
    ContinentIndex::new(self)
  }
}
