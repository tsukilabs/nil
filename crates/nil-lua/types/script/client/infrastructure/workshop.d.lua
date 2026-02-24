-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@param req AddWorkshopRecruitOrderRequest
function client:addWorkshopRecruitOrder(req) end

---@param req CancelWorkshopRecruitOrderRequest
function client:cancelWorkshopRecruitOrder(req) end

---@param req GetWorkshopRecruitCatalogRequest
---@return WorkshopRecruitCatalog
function client:getWorkshopRecruitCatalog(req) end
