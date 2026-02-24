-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class Prefecture: Building
---@field buildQueue PrefectureBuildQueue

---@class PrefectureBuildQueue
---@field orders PrefectureBuildOrder[]

---@class PrefectureBuildOrder: InfrastructureQueueOrder
---@field kind PrefectureBuildOrderKind
---@field building BuildingId
---@field level BuildingLevel

---@alias PrefectureBuildOrderKind "construction" | "demolition"

---@class PrefectureBuildOrderRequest
---@field coord Coord
---@field building BuildingId
---@field kind PrefectureBuildOrderKind

---@alias PrefectureBuildCatalog table

---@alias PrefectureBuildCatalogEntry
---| PrefectureBuildCatalogEntryAvailable
---| PrefectureBuildCatalogEntryMaxed
---| PrefectureBuildCatalogEntryUnmet

---@class PrefectureBuildCatalogEntryAvailable
---@field kind "available"
---@field recipe PrefectureBuildCatalogRecipe

---@class PrefectureBuildCatalogEntryMaxed
---@field kind "maxed"

---@class PrefectureBuildCatalogEntryUnmet
---@field kind "unmet"
---@field requirements InfrastructureRequirements

---@class PrefectureBuildCatalogRecipe
---@field level BuildingLevel
---@field resources Resources
---@field maintenance number
---@field workforce number
---@field requirements InfrastructureRequirements
