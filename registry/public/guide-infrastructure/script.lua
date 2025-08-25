-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

--- Enables or disables a building of a city you own.
-- @param {table} coord
-- @param {string} building_id
-- @param {boolean} enabled
world:toggle_building({ x = 0, y = 0 }, "iron-mine", false)
