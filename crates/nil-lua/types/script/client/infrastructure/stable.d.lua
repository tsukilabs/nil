-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@param req AddStableRecruitOrderRequest
function client:addStableRecruitOrder(req) end

---@param req CancelStableRecruitOrderRequest
function client:cancelStableRecruitOrder(req) end

---@param req GetStableRecruitCatalogRequest
---@return StableRecruitCatalog
function client:getStableRecruitCatalog(req) end
