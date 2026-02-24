-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@param req CheatGetEthicsRequest
---@return Ethics|nil
function client:cheatGetEthics(req) end

---@param req CheatSetBotEthicsRequest
function client:cheatSetBotEthics(req) end

---@param req CheatSpawnBotRequest
---@return BotId
function client:cheatSpawnBot(req) end
