-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@param req GetPlayerRequest
---@return Player
function client:getPlayer(req) end

---@param req GetPlayerCoordsRequest
---@return Coord[]
function client:getPlayerCoords(req) end

---@param req GetPlayerMaintenanceRequest
---@return number
function client:getPlayerMaintenance(req) end

---@param req GetPlayerMilitaryRequest
---@return table
function client:getPlayerMilitary(req) end

---@param req GetPlayerReportsRequest
---@return ReportId[]
function client:getPlayerReports(req) end

---@param req GetPlayerStatusRequest
---@return PlayerStatus
function client:getPlayerStatus(req) end

---@param req GetPlayerWorldsRequest
---@return WorldId[]
function client:getPlayerWorlds(req) end

---@param req GetPlayerStorageCapacityRequest
---@return OverallStorageCapacity
function client:getPlayerStorageCapacity(req) end

---@param req GetPublicPlayerRequest
---@return PublicPlayer
function client:getPublicPlayer(req) end

---@param req GetPublicPlayersRequest
---@return PublicPlayer[]
function client:getPublicPlayers(req) end

---@param req SetPlayerStatusRequest
function client:setPlayerStatus(req) end

---@param req SpawnPlayerRequest
function client:spawnPlayer(req) end

---@param req PlayerExistsRequest
---@return boolean
function client:playerExists(req) end
