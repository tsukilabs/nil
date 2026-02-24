-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@param req CreateRemoteWorldRequest
---@return WorldId
function client:createRemoteWorld(req) end

---@param req GetRemoteWorldRequest
---@return RemoteWorld
function client:getRemoteWorld(req) end

---@return RemoteWorld[]
function client:getRemoteWorlds() end

---@param req GetWorldBotsRequest
---@return BotId[]
function client:getWorldBots(req) end

---@param req GetWorldConfigRequest
---@return WorldConfig
function client:getWorldConfig(req) end

---@param req GetWorldPlayersRequest
---@return PlayerId[]
function client:getWorldPlayers(req) end

---@param req GetWorldPrecursorsRequest
---@return PrecursorId[]
function client:getWorldPrecursors(req) end

---@param req GetWorldStatsRequest
---@return WorldStats
function client:getWorldStats(req) end

---@param req SaveLocalWorldRequest
function client:saveLocalWorld(req) end
