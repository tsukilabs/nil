// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::battle::{Battle, BattleResult};
use crate::continent::Coord;
use crate::error::Result;
use crate::infrastructure::building::{Building, BuildingLevel};
use crate::military::army::{Army, ArmyState};
use crate::military::maneuver::{Maneuver, ManeuverDirection, ManeuverKind};
use crate::military::unit::stats::haul::Haul;
use crate::player::PlayerId;
use crate::report::{BattleReport, Report};
use crate::resources::Resources;
use crate::ruler::Ruler;
use crate::world::World;
use itertools::Itertools;
use nil_util::result::WrapOk;
use nil_util::vec::VecExt;
use num_traits::ToPrimitive;

impl World {
  pub(super) fn process_maneuvers(&mut self) -> Result<()> {
    for maneuver in self.military.advance_maneuvers()? {
      debug_assert!(maneuver.is_done());
      match maneuver.direction() {
        ManeuverDirection::Going => self.process_going_maneuver(maneuver)?,
        ManeuverDirection::Returning => self.process_returning_maneuver(&maneuver)?,
      }
    }

    Ok(())
  }

  fn process_going_maneuver(&mut self, mut maneuver: Maneuver) -> Result<()> {
    let army_id = maneuver.army();
    let destination = maneuver.destination();
    let rulers = ManeuverRulers::new(self, &maneuver)?;

    match maneuver.kind() {
      // TODO: Calculate defender losses.
      ManeuverKind::Attack => {
        let battle_result = perform_battle(self, &maneuver)?;
        let attacker_surviving_personnel = battle_result.attacker_surviving_personnel();

        if attacker_surviving_personnel.is_empty() {
          self.military.remove_army(army_id)?;
        } else {
          maneuver.reverse()?;
          self.military.insert_maneuver(maneuver);

          let army = self.military.army_mut(army_id)?;
          *army.personnel_mut() = attacker_surviving_personnel.clone();

          let haul = army.haul();
          let hauled_resources = calculate_hauled_resources(self, destination, haul)?;

          if !hauled_resources.is_empty() {
            self.transpose_resources(
              rulers.destination_ruler.clone(),
              rulers.sender.clone(),
              hauled_resources.clone(),
            )?;
          }

          let players = rulers.players();
          let report = BattleReport::builder()
            .attacker(rulers.sender)
            .defender(rulers.destination_ruler)
            .hauled_resources(hauled_resources)
            .result(battle_result)
            .build();

          emit_battle_report(self, &report);
          self.report.manage(report.into(), players);
        }
      }
      ManeuverKind::Support => {
        self
          .military
          .relocate_army(army_id, destination)?;
      }
    }

    Ok(())
  }

  fn process_returning_maneuver(&mut self, maneuver: &Maneuver) -> Result<()> {
    let army = self.military.army_mut(maneuver.army())?;
    *army.state_mut() = ArmyState::Idle;
    Ok(())
  }
}

struct ManeuverRulers {
  sender: Ruler,
  destination_ruler: Ruler,
  destination_army_owners: Box<[Ruler]>,
}

impl ManeuverRulers {
  fn new(world: &World, maneuver: &Maneuver) -> Result<Self> {
    let sender = world
      .military
      .army(maneuver.army())?
      .owner()
      .clone();

    let destination_ruler = world
      .city(maneuver.destination())?
      .owner()
      .clone();

    let mut destination_army_owners = Vec::new();
    if let ManeuverKind::Attack = maneuver.kind() {
      let owners = world
        .military
        .idle_armies_at(maneuver.destination())
        .map(Army::owner)
        .unique()
        .cloned();

      destination_army_owners.extend(owners);
    }

    destination_army_owners.retain(|it| it != &sender && it != &destination_ruler);

    Ok(Self {
      sender,
      destination_ruler,
      destination_army_owners: destination_army_owners.into_boxed_slice(),
    })
  }

  fn players(&self) -> Vec<PlayerId> {
    let mut players = Vec::new();
    players.try_push(self.sender.player().cloned());
    players.try_push(self.destination_ruler.player().cloned());
    players
  }
}

fn perform_battle(world: &World, maneuver: &Maneuver) -> Result<BattleResult> {
  let attacker = world.military.squads(maneuver.army())?;
  let defender = world
    .military
    .idle_squads_at(maneuver.destination());

  let wall = world
    .infrastructure(maneuver.destination())?
    .wall()
    .level();

  let wall_stats = (wall > BuildingLevel::ZERO)
    .then(|| world.stats.infrastructure.wall().get(wall))
    .transpose()?;

  Battle::builder()
    .attacker(&attacker)
    .defender(&defender)
    .maybe_wall(wall_stats)
    .build()
    .result()
    .wrap_ok()
}

fn calculate_hauled_resources(world: &World, target: Coord, base: Haul) -> Result<Resources> {
  let resources = world.get_weighted_resources(target)?;
  let silo_resources = resources.sum_silo();
  let warehouse_resources = resources.sum_warehouse();

  let mut hauled = Resources::new();
  let mut silo_haul = base * Haul::SILO_RATIO;
  let mut warehouse_haul = base * Haul::WAREHOUSE_RATIO;

  if silo_haul > silo_resources {
    silo_haul = Haul::new(silo_resources);
  }

  if warehouse_haul > warehouse_resources {
    warehouse_haul = Haul::new(warehouse_resources);
  }

  macro_rules! set {
    ($total:expr, $res:ident, $haul:expr) => {
      let total = f64::from($total);
      if total.is_normal() {
        let ratio = f64::from(resources.$res) / total;
        let resource = (f64::from($haul) * ratio)
          .round()
          .to_u32()
          .unwrap_or_default();

        hauled.$res = resource.min(*resources.$res).into();
      }
    };
  }

  set!(silo_resources, food, silo_haul);
  set!(warehouse_resources, iron, warehouse_haul);
  set!(warehouse_resources, stone, warehouse_haul);
  set!(warehouse_resources, wood, warehouse_haul);

  Ok(hauled)
}

fn emit_battle_report(world: &World, report: &BattleReport) {
  if let Some(attacker) = report.attacker().player().cloned() {
    world.emit_report(attacker, report.id());
  }

  if let Some(defender) = report.defender().player().cloned() {
    debug_assert_ne!(report.attacker().player(), Some(&defender));
    world.emit_report(defender, report.id());
  }
}
