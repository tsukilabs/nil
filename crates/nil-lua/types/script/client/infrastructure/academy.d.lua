-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@param req AddAcademyRecruitOrderRequest
function client:addAcademyRecruitOrder(req) end

---@param req CancelAcademyRecruitOrderRequest
function client:cancelAcademyRecruitOrder(req) end

---@param req GetAcademyRecruitCatalogRequest
---@return AcademyRecruitCatalog
function client:getAcademyRecruitCatalog(req) end
