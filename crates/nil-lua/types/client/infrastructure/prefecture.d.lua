-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@param req AddPrefectureBuildOrderRequest
function client:addPrefectureBuildOrder(req) end

---@param req CancelPrefectureBuildOrderRequest
function client:cancelPrefectureBuildOrder(req) end

---@param req GetPrefectureBuildCatalogRequest
---@return PrefectureBuildCatalog
function client:getPrefectureBuildCatalog(req) end
