// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::PrefectureBuildOrderKind::{Construction, Demolition};
use super::{PrefectureBuildOrderKind, PrefectureBuildOrderRequest};
use crate::continent::Coord;
use crate::error::Error;
use crate::infrastructure::building::{BuildingId, BuildingStatsTable};
use crate::infrastructure::{Infrastructure, InfrastructureStats};
use crate::resources::Resources;
use std::sync::LazyLock;

static STATS: LazyLock<InfrastructureStats> = LazyLock::new(InfrastructureStats::default);

#[test]
fn cannot_decrease() {
  let mut infrastructure = Infrastructure::default();
  infrastructure
    .building_mut(BuildingId::Prefecture)
    .set_min_level();

  assert!(
    infrastructure
      .add_prefecture_build_order(stats(), None, &req(Demolition))
      .is_err_and(|err| matches!(err, Error::CannotDecreaseBuildingLevel(_)))
  );
}

#[test]
fn cannot_increase() {
  let mut infrastructure = Infrastructure::default();
  infrastructure
    .building_mut(BuildingId::Prefecture)
    .set_max_level();

  assert!(
    infrastructure
      .add_prefecture_build_order(stats(), None, &req(Construction))
      .is_err_and(|err| matches!(err, Error::CannotIncreaseBuildingLevel(_)))
  );
}

#[test]
fn insufficient_resources() {
  assert!(
    Infrastructure::default()
      .add_prefecture_build_order(stats(), Some(&Resources::MIN), &req(Construction))
      .is_err_and(|err| matches!(err, Error::InsufficientResources))
  );
}

#[test]
fn has_resources() {
  Infrastructure::default()
    .add_prefecture_build_order(stats(), Some(&Resources::MAX), &req(Construction))
    .expect("should have enough resources");
}

#[test]
fn cancel_build_order() {
  let mut infrastructure = Infrastructure::default();
  infrastructure
    .add_prefecture_build_order(stats(), None, &req(Construction))
    .unwrap();

  let order = infrastructure
    .cancel_prefecture_build_order()
    .expect("should have an order in the queue");

  assert_eq!(order.kind(), Construction);
  assert_eq!(order.building(), BuildingId::Prefecture);
  assert!(
    infrastructure
      .prefecture
      .build_queue
      .is_empty()
  );
}

fn stats() -> &'static BuildingStatsTable {
  STATS
    .building(BuildingId::Prefecture)
    .unwrap()
}

fn req(kind: PrefectureBuildOrderKind) -> PrefectureBuildOrderRequest {
  PrefectureBuildOrderRequest {
    coord: Coord::splat(0),
    building: BuildingId::Prefecture,
    kind,
  }
}
