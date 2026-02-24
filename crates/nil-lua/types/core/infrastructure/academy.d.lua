-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class Academy: Building
---@field recruitQueue AcademyRecruitQueue

---@class AcademyRecruitQueue
---@field orders AcademyRecruitOrder[]

---@class AcademyRecruitOrder: InfrastructureQueueOrder
---@field squad Squad

---@class AcademyRecruitOrderRequest
---@field coord Coord
---@field unit AcademyUnitId
---@field chunks number

---@alias AcademyRecruitCatalog table

---@alias AcademyRecruitCatalogEntry
---| AcademyRecruitCatalogEntryAvailable
---| AcademyRecruitCatalogEntryUnmet

---@class AcademyRecruitCatalogEntryAvailable
---@field kind "available"
---@field recipe AcademyRecruitCatalogRecipe

---@class AcademyRecruitCatalogEntryUnmet
---@field kind "unmet"
---@field requirements InfrastructureRequirements

---@class AcademyRecruitCatalogRecipe
---@field resources Resources
---@field maintenance number
---@field workforce number
---@field requirements InfrastructureRequirements
