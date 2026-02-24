-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@alias Ruler RulerBot | RulerPlayer | RulerPrecursor

---@alias RulerId BotId | PlayerId | PrecursorId
---@alias RulerKind 'bot' | 'player' | 'precursor'

---@class RulerBot
---@field kind 'bot'
---@field id BotId

---@class RulerPlayer
---@field kind 'player'
---@field id PlayerId

---@class RulerPrecursor
---@field kind 'precursor'
---@field id PrecursorId
