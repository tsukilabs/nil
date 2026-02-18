-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class CheatGetAcademyRecruitQueueRequest
---@field world WorldId
---@field coord Coord

---@class CheatGetAcademyRecruitQueuesRequest
---@field world WorldId
---@field coords Coord[]
---@field filterEmpty boolean

---@class CheatGetAllAcademyRecruitQueuesRequest
---@field world WorldId
---@field filterEmpty boolean

---@class CheatGetAllPrefectureBuildQueuesRequest
---@field world WorldId
---@field filterEmpty boolean

---@class CheatGetAllStableRecruitQueuesRequest
---@field world WorldId
---@field filterEmpty boolean

---@class CheatGetInfrastructureRequest
---@field world WorldId
---@field coord Coord

---@class CheatGetPrefectureBuildQueueRequest
---@field world WorldId
---@field coord Coord

---@class CheatGetPrefectureBuildQueuesRequest
---@field world WorldId
---@field coords Coord[]
---@field filterEmpty boolean

---@class CheatGetStableRecruitQueueRequest
---@field world WorldId
---@field coord Coord

---@class CheatGetStableRecruitQueuesRequest
---@field world WorldId
---@field coords Coord[]
---@field filterEmpty boolean

---@class CheatGetStorageCapacityRequest
---@field world WorldId
---@field ruler Ruler|nil

---@class CheatSetBuildingLevelRequest
---@field world WorldId
---@field coord Coord
---@field id BuildingId
---@field level BuildingLevel

---@class CheatSetMaxInfrastructureRequest
---@field world WorldId
---@field coord Coord
