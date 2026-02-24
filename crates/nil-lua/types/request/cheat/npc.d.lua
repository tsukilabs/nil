-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class CheatGetEthicsRequest
---@field world WorldId
---@field ruler Ruler

---@class CheatSetBotEthicsRequest
---@field world WorldId
---@field id BotId
---@field ethics Ethics

---@class CheatSpawnBotRequest
---@field world WorldId
---@field name string
---@field infrastructure Infrastructure|nil
