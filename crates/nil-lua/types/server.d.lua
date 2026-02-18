-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class LocalServer
---@field worldId WorldId
---@field addr string

---@alias ServerAddr ServerAddrLocal | ServerAddrRemote

---@class ServerAddrLocal
---@field kind 'local'
---@field addr string

---@class ServerAddrRemote
---@field kind 'remote'

---@alias ServerKind ServerKindLocal | ServerKindRemote

---@class ServerKindLocal
---@field kind 'local'
---@field id WorldId

---@class ServerKindRemote
---@field kind 'remote'

---@class RemoteWorld
---@field config WorldConfig
---@field description string|nil
---@field createdBy PlayerId
---@field createdAt string
---@field updatedAt string
---@field hasPassword boolean
---@field currentRound RoundId
---@field activePlayers number
---@field totalPlayers number
---@field continentSize ContinentSize
