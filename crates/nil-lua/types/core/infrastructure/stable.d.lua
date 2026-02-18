-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class Stable: Building
---@field recruitQueue StableRecruitQueue

---@class StableRecruitQueue
---@field orders StableRecruitOrder[]

---@class StableRecruitOrder: InfrastructureQueueOrder
---@field squad Squad

---@class StableRecruitOrderRequest
---@field coord Coord
---@field unit StableUnitId
---@field chunks number

---@alias StableRecruitCatalog table

---@alias StableRecruitCatalogEntry
---| StableRecruitCatalogEntryAvailable
---| StableRecruitCatalogEntryUnmet

---@class StableRecruitCatalogEntryAvailable
---@field kind "available"
---@field recipe StableRecruitCatalogRecipe

---@class StableRecruitCatalogEntryUnmet
---@field kind "unmet"
---@field requirements InfrastructureRequirements

---@class StableRecruitCatalogRecipe
---@field resources Resources
---@field maintenance number
---@field workforce number
---@field requirements InfrastructureRequirements
