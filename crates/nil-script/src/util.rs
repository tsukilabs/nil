// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use itertools::Itertools;
use std::iter::repeat_n;

pub fn u8_array_to_usize<const N: usize>(array: [u8; N]) -> usize {
  debug_assert!(N <= 8);
  array
    .into_iter()
    .chain(repeat_n(0u8, 8 - N))
    .collect_array::<8>()
    .map(usize::from_le_bytes)
    .expect("iterator has exactly eight values")
}
