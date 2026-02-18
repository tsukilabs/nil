-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@alias PublicField PublicFieldEmpty | PublicFieldCity

---@alias PublicFieldKind 'empty' | 'city'

---@class PublicFieldEmpty
---@field kind 'empty'

---@class PublicFieldCity
---@field kind 'city'
---@field city PublicCity

---@class Coord
---@field x number
---@field y number

---@alias ContinentKey Coord | ContinentIndex
---@alias ContinentIndex number
---@alias ContinentSize number

---@class CitySearch
---@field coord Coord[]|nil
---@field name string[]|nil
