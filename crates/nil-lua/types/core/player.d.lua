-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class PublicPlayer
---@field id PlayerId
---@field status PlayerStatus

---@class Player: PublicPlayer
---@field resources Resources

---@alias PlayerId string

---@alias PlayerStatus 'active' | 'inactive'

---@class PlayerOptions
---@field id PlayerId
