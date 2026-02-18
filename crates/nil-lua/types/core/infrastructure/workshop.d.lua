-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class Workshop: Building
---@field recruitQueue WorkshopRecruitQueue

---@class WorkshopRecruitQueue
---@field orders WorkshopRecruitOrder[]

---@class WorkshopRecruitOrder: InfrastructureQueueOrder
---@field squad Squad

---@class WorkshopRecruitOrderRequest
---@field coord Coord
---@field unit WorkshopUnitId
---@field chunks number

---@alias WorkshopRecruitCatalog table

---@alias WorkshopRecruitCatalogEntry
---| WorkshopRecruitCatalogEntryAvailable
---| WorkshopRecruitCatalogEntryUnmet

---@class WorkshopRecruitCatalogEntryAvailable
---@field kind "available"
---@field recipe WorkshopRecruitCatalogRecipe

---@class WorkshopRecruitCatalogEntryUnmet
---@field kind "unmet"
---@field requirements InfrastructureRequirements

---@class WorkshopRecruitCatalogRecipe
---@field resources Resources
---@field maintenance number
---@field workforce number
---@field requirements InfrastructureRequirements
