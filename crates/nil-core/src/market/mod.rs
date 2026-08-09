// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod fee;
pub mod vault;

use crate::market::fee::MarketFee;
use crate::market::vault::MarketVault;
use crate::resources::gold::Gold;
use crate::resources::{Food, Iron, Resources, Stone, Wood};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[derive_const(Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Market {
  pub(crate) vault: MarketVault,
  pub(crate) fee: MarketFee,
}

impl Market {
  pub const fn new(fee: MarketFee) -> Self {
    Self {
      vault: MarketVault::default(),
      fee: fee.clamped(),
    }
  }

  #[inline]
  pub const fn vault(&self) -> &MarketVault {
    &self.vault
  }

  #[inline]
  pub const fn fee(&self) -> MarketFee {
    self.fee
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

  /// Maximum amount of a resource that can be bought with the given amount of gold.
  pub fn buyable_amount(&self, market_price: Gold, gold: Gold) -> u32 {
    let fee = f64::from(self.fee());
    let market_price = f64::from(market_price);
    let gold = f64::from(gold);
    let resource = gold / (market_price * (1.0 + fee));
    resource.floor().max(0.0) as u32
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
