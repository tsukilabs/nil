// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_not_owned_by;
use crate::world::script::WorldUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};

#[expect(clippy::too_many_lines)]
pub(super) fn add_methods<'a, M: UserDataMethods<WorldUserData<'a>>>(methods: &mut M) {
  methods.add_method_mut(
    "cheat_set_building_level",
    |lua, this, (coord, id, level): (Value, Value, Value)| {
      let coord = lua.from_value(coord)?;
      let id = lua.from_value(id)?;
      let level = lua.from_value(level)?;
      bail_not_owned_by!(this, coord);
      this
        .world
        .cheat_set_building_level(coord, id, level)
        .map_err(Into::into)
    },
  );

  methods.add_method_mut("cheat_set_food", |lua, this, food: Value| {
    let food = lua.from_value(food)?;
    this
      .world
      .cheat_set_food(this.player.clone(), food)
      .map_err(Into::into)
  });

  methods.add_method_mut("cheat_set_iron", |lua, this, iron: Value| {
    let iron = lua.from_value(iron)?;
    this
      .world
      .cheat_set_iron(this.player.clone(), iron)
      .map_err(Into::into)
  });

  methods.add_method_mut("cheat_set_max_food", |_, this, ()| {
    this
      .world
      .cheat_set_max_food(this.player.clone())
      .map_err(Into::into)
  });

  methods.add_method_mut("cheat_set_max_infrastructure", |lua, this, coord: Value| {
    let coord = lua.from_value(coord)?;
    bail_not_owned_by!(this, coord);
    this
      .world
      .cheat_set_max_infrastructure(coord)
      .map_err(Into::into)
  });

  methods.add_method_mut("cheat_set_max_iron", |_, this, ()| {
    this
      .world
      .cheat_set_max_iron(this.player.clone())
      .map_err(Into::into)
  });

  methods.add_method_mut("cheat_set_max_resources", |_, this, ()| {
    this
      .world
      .cheat_set_max_resources(this.player.clone())
      .map_err(Into::into)
  });

  methods.add_method_mut("cheat_set_max_silo_resources", |_, this, ()| {
    this
      .world
      .cheat_set_max_silo_resources(this.player.clone())
      .map_err(Into::into)
  });

  methods.add_method_mut("cheat_set_max_stone", |_, this, ()| {
    this
      .world
      .cheat_set_max_stone(this.player.clone())
      .map_err(Into::into)
  });

  methods.add_method_mut("cheat_set_max_warehouse_resources", |_, this, ()| {
    this
      .world
      .cheat_set_max_warehouse_resources(this.player.clone())
      .map_err(Into::into)
  });

  methods.add_method_mut("cheat_set_max_wood", |_, this, ()| {
    this
      .world
      .cheat_set_max_wood(this.player.clone())
      .map_err(Into::into)
  });

  methods.add_method_mut("cheat_set_resources", |lua, this, resources: Value| {
    let resources = lua.from_value(resources)?;
    this
      .world
      .cheat_set_resources(this.player.clone(), resources)
      .map_err(Into::into)
  });

  methods.add_method_mut(
    "cheat_set_stability",
    |lua, this, (coord, stability): (Value, Value)| {
      let coord = lua.from_value(coord)?;
      let stability = lua.from_value(stability)?;
      bail_not_owned_by!(this, coord);
      this
        .world
        .cheat_set_stability(coord, stability)
        .map_err(Into::into)
    },
  );

  methods.add_method_mut("cheat_set_stone", |lua, this, stone: Value| {
    let stone = lua.from_value(stone)?;
    this
      .world
      .cheat_set_stone(this.player.clone(), stone)
      .map_err(Into::into)
  });

  methods.add_method_mut("cheat_set_wood", |lua, this, wood: Value| {
    let wood = lua.from_value(wood)?;
    this
      .world
      .cheat_set_wood(this.player.clone(), wood)
      .map_err(Into::into)
  });
}
