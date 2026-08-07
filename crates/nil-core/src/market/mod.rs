// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod fee;
pub mod vault;

use crate::market::fee::MarketFee;
use crate::market::vault::MarketVault;
use crate::resources::gold::Gold;
use crate::resources::{Food, Iron, Resources, Stone, Wood};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[derive_const(Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Market {
  vault: MarketVault,
  fee: MarketFee,

  // We don't really need to include the market price here,
  // since it can be derived from the resources themselves.
  // However, including it simplifies things for downstream consumers,
  // as they don't need to query the resources separately to obtain it.
  #[cfg_attr(feature = "typescript", ts(as = "MarketPriceTable"))]
  #[cfg_attr(
    not(feature = "typescript"),
    serde(skip_deserializing, serialize_with = "serialize_market_price_table")
  )]
  price_table: PhantomData<MarketPriceTable>,
}

impl Market {
  pub const fn new(fee: MarketFee) -> Self {
    Self {
      vault: MarketVault::default(),
      fee: fee.clamped(),
      price_table: PhantomData,
    }
  }

  #[inline]
  pub const fn vault(&self) -> &MarketVault {
    &self.vault
  }

  pub(crate) const fn vault_mut(&mut self) -> &mut MarketVault {
    &mut self.vault
  }

  #[inline]
  pub const fn fee(&self) -> MarketFee {
    self.fee
  }

  /// Sets the market fee, clamping it to the valid range.
  ///
  /// As the fee is not expected to be changed throughout the game,
  /// this should only be used to execute cheats or for testing purposes.
  pub(crate) const fn set_fee(&mut self, fee: MarketFee) {
    self.fee = fee.clamped();
  }

  #[inline]
  pub const fn price_table(&self) -> MarketPriceTable {
    MarketPriceTable::default()
  }

  #[inline]
  pub const fn price_of(&self, op: MarketOperation, resources: Resources) -> Gold {
    match op {
      MarketOperation::Buy => Gold::from(resources + (resources * self.fee())),
      MarketOperation::Sell => Gold::from(resources),
    }
  }
}

#[derive(Copy, Debug, strum::Display, Hash, Deserialize, Serialize)]
#[derive_const(Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum MarketOperation {
  Buy,
  Sell,
}

#[derive(Copy, Debug, Deserialize, Serialize)]
#[derive_const(Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct MarketPriceTable {
  food: Gold,
  iron: Gold,
  stone: Gold,
  wood: Gold,
}

impl MarketPriceTable {
  pub const fn new() -> Self {
    Self::default()
  }
}

const impl Default for MarketPriceTable {
  fn default() -> Self {
    Self {
      food: Food::MARKET_PRICE,
      iron: Iron::MARKET_PRICE,
      stone: Stone::MARKET_PRICE,
      wood: Wood::MARKET_PRICE,
    }
  }
}

#[cfg(not(feature = "typescript"))]
#[allow(clippy::trivially_copy_pass_by_ref)]
fn serialize_market_price_table<S>(
  _: &PhantomData<MarketPriceTable>,
  serializer: S,
) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  MarketPriceTable::default().serialize(serializer)
}
