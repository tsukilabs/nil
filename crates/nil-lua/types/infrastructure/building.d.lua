-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@alias BuildingId
---| 'academy'
---| 'farm'
---| 'iron-mine'
---| 'prefecture'
---| 'quarry'
---| 'sawmill'
---| 'silo'
---| 'stable'
---| 'wall'
---| 'warehouse'
---| 'workshop'

---@alias BuildingLevel number

---@class Building
---@field level BuildingLevel
---@field enabled boolean

---@alias Sawmill Building
---@alias Quarry Building
---@alias IronMine Building
---@alias Farm Building
---@alias Warehouse Building
---@alias Silo Building

---@class BuildingStats
---@field level BuildingLevel
---@field cost number
---@field resources Resources
---@field maintenance number
---@field workforce number
