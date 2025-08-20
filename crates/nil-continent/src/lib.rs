// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use std::cmp::Ordering;
use std::mem;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default)]
pub struct Continent {
  container: Container,
  coords: Vec<Coord>,
  center: Coord,
}

#[wasm_bindgen]
impl Continent {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self::default()
  }

  #[wasm_bindgen]
  pub fn center(&self) -> Coord {
    self.center
  }

  #[wasm_bindgen]
  pub fn set_center(&mut self, x: i16, y: i16) {
    self.center = Coord(x, y);
  }

  #[wasm_bindgen]
  pub fn max_cols(&self) -> u32 {
    self.container.cols()
  }

  #[wasm_bindgen]
  pub fn max_rows(&self) -> u32 {
    self.container.rows()
  }

  #[wasm_bindgen]
  pub fn set_container_size(&mut self, width: u32, height: u32) {
    self.container.width = width;
    self.container.height = height;
  }

  #[wasm_bindgen]
  pub fn set_cell_size(&mut self, size: u32) {
    self.container.cell_size = size;
  }

  #[wasm_bindgen]
  pub fn update_coords_within(&mut self) -> *const Coord {
    let x = self.center.0;
    let y = self.center.1;
    let rows = self.container.rows() as i16;
    let cols = self.container.cols() as i16;

    let x_min = x - cols / 2;
    let x_max = x + cols / 2;
    let y_min = y - rows / 2;
    let y_max = y + rows / 2;

    self.coords.clear();
    for x in x_min..=x_max {
      for y in y_min..=y_max {
        self.coords.push(Coord::new(x, y));
      }
    }

    self.coords.sort();
    self.coords.as_ptr()
  }

  #[wasm_bindgen]
  pub fn coords_byte_length(&self) -> usize {
    Coord::type_size() * self.coords.len()
  }
}

pub struct Container {
  width: u32,
  height: u32,
  cell_size: u32,
}

impl Container {
  fn cols(&self) -> u32 {
    self
      .width
      .div_ceil(self.cell_size)
      .next_multiple_of(2)
      .saturating_add(1)
  }

  fn rows(&self) -> u32 {
    self
      .height
      .div_ceil(self.cell_size)
      .next_multiple_of(2)
      .saturating_add(1)
  }
}

impl Default for Container {
  fn default() -> Self {
    Self { width: 0, height: 0, cell_size: 50 }
  }
}

#[repr(C)]
#[wasm_bindgen]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Coord(i16, i16);

#[wasm_bindgen]
impl Coord {
  #[wasm_bindgen(constructor)]
  pub fn new(x: i16, y: i16) -> Self {
    Self(x, y)
  }

  #[wasm_bindgen]
  pub fn type_size() -> usize {
    mem::size_of::<Self>()
  }

  #[wasm_bindgen]
  pub fn x(&self) -> i16 {
    self.0
  }

  #[wasm_bindgen]
  pub fn y(&self) -> i16 {
    self.1
  }
}

impl PartialOrd for Coord {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Coord {
  fn cmp(&self, other: &Self) -> Ordering {
    other
      .1
      .cmp(&self.1)
      .then(self.0.cmp(&other.0))
  }
}
