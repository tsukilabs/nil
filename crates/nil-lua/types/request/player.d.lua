-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class GetPlayerRequest
---@field world WorldId
---@field id PlayerId

---@class GetPlayerCoordsRequest
---@field world WorldId
---@field id PlayerId

---@class GetPlayersRequest
---@field world WorldId

---@class GetPlayerMaintenanceRequest
---@field world WorldId

---@class GetPlayerMilitaryRequest
---@field world WorldId

---@class GetPlayerReportsRequest
---@field world WorldId

---@class GetPlayerStatusRequest
---@field world WorldId
---@field id PlayerId

---@class GetPlayerStorageCapacityRequest
---@field world WorldId

---@class GetPlayerWorldsRequest
---@field id PlayerId

---@class GetPublicPlayerRequest
---@field world WorldId
---@field id PlayerId

---@class GetPublicPlayersRequest
---@field world WorldId

---@class PlayerExistsRequest
---@field world WorldId
---@field id PlayerId

---@class SetPlayerStatusRequest
---@field world WorldId
---@field status PlayerStatus

---@class SpawnPlayerRequest
---@field world WorldId
---@field worldPassword string|nil
---@field options PlayerOptions
