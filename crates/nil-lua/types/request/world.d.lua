-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class CreateRemoteWorldRequest
---@field options WorldOptions
---@field description string|nil
---@field password string|nil
---@field roundDuration RoundDuration|nil

---@class DeleteRemoteWorldRequest
---@field world WorldId

---@class GetRemoteWorldRequest
---@field world WorldId

---@class GetWorldBotsRequest
---@field world WorldId

---@class GetWorldConfigRequest
---@field world WorldId

---@class GetWorldPlayersRequest
---@field world WorldId

---@class GetWorldPrecursorsRequest
---@field world WorldId

---@class GetWorldStatsRequest
---@field world WorldId

---@class SaveLocalWorldRequest
---@field world WorldId
---@field path string
