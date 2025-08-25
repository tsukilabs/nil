-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

--- Gets the amount of resources that a bot has.
-- @param {number} bot_id
-- @return {table}
world:cheat_get_bot_resources(1)

--- Gets the total storage capacity of a bot.
-- @param {number} bot_id
-- @return {table}
world:cheat_get_bot_storage_capacity(1)

--- Gets the amount of resources that a precursor has.
-- @param {string} precursor_id
-- @return {table}
world:cheat_get_precursor_resources("A")

--- Gets the total storage capacity of a precursor.
-- @param {string} precursor_id
-- @return {table}
world:cheat_get_precursor_storage_capacity("A")

--- Sets the level of a building.
-- @param {table} coord
-- @param {string} building_id
-- @param {number} level
world:cheat_set_building_level({ x = 0, y = 0 }, "farm", 30)

--- Sets the amount of food you have.
-- @param {number} food
world:cheat_set_food(10000)

--- Sets the amount of iron you have.
-- @param {number} iron
world:cheat_set_iron(10000)

--- Sets your food to the maximum amount possible.
world:cheat_set_max_food()

--- Sets the levels of all buildings in a city to their maximum.
-- @param {table} coord
world:cheat_set_max_infrastructure({ x = 0, y = 0 })

--- Sets your iron to the maximum amount possible.
world:cheat_set_max_iron()

--- Sets your resources to the maximum possible amount.
world:cheat_set_max_resources()

--- Sets your silo resources to the maximum possible amount.
world:cheat_set_max_silo_resources()

--- Sets your stone to the maximum amount possible.
world:cheat_set_max_stone()

--- Sets your warehouse resources to the maximum possible amount.
world:cheat_set_max_warehouse_resources()

--- Sets your wood to the maximum amount possible.
world:cheat_set_max_wood()

--- Sets your current resources.
-- @param {table} resources
world:cheat_set_resources({ food = 1000, iron = 1000, stone = 1000, wood = 1000 })

--- Sets the stability of a city.
-- @param {table} coord
-- @param {number} stability
world:cheat_set_stability({ x = 0, y = 0 }, 0.5)

--- Sets the amount of stone you have.
-- @param {number} stone
world:cheat_set_stone(10000)

--- Sets the amount of wood you have.
-- @param {number} wood
world:cheat_set_wood(10000)

--- Spawns a bot.
-- @param {name} Bot name
-- @return {number} Bot id
world:cheat_spawn_bot("C-3PO")
