-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@param req CheatGetAcademyRecruitQueueRequest
---@return AcademyRecruitQueue
function client:cheatGetAcademyRecruitQueue(req) end

---@param req CheatGetAcademyRecruitQueuesRequest
---@return [Coord, AcademyRecruitQueue][]
function client:cheatGetAcademyRecruitQueues(req) end

---@param req CheatGetAllAcademyRecruitQueuesRequest
---@return [Coord, AcademyRecruitQueue][]
function client:cheatGetAllAcademyRecruitQueues(req) end

---@param req CheatGetAllPrefectureBuildQueuesRequest
---@return [Coord, PrefectureBuildQueue][]
function client:cheatGetAllPrefectureBuildQueues(req) end

---@param req CheatGetAllStableRecruitQueuesRequest
---@return [Coord, StableRecruitQueue][]
function client:cheatGetAllStableRecruitQueues(req) end

---@param req CheatGetInfrastructureRequest
---@return Infrastructure
function client:cheatGetInfrastructure(req) end

---@param req CheatGetPrefectureBuildQueueRequest
---@return PrefectureBuildQueue
function client:cheatGetPrefectureBuildQueue(req) end

---@param req CheatGetPrefectureBuildQueuesRequest
---@return [Coord, PrefectureBuildQueue][]
function client:cheatGetPrefectureBuildQueues(req) end

---@param req CheatGetStableRecruitQueueRequest
---@return StableRecruitQueue
function client:cheatGetStableRecruitQueue(req) end

---@param req CheatGetStableRecruitQueuesRequest
---@return [Coord, StableRecruitQueue][]
function client:cheatGetStableRecruitQueues(req) end

---@param req CheatGetStorageCapacityRequest
---@return OverallStorageCapacity
function client:cheatGetStorageCapacity(req) end

---@param req CheatSetBuildingLevelRequest
function client:cheatSetBuildingLevel(req) end

---@param req CheatSetMaxInfrastructureRequest
function client:cheatSetMaxInfrastructure(req) end
