-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@param req GetContinentSizeRequest
---@return ContinentSize
function client:getContinentSize(req) end

---@param req GetPublicFieldRequest
---@return PublicField
function client:getPublicField(req) end

---@param req GetPublicFieldsRequest
---@return [Coord, PublicField][]
function client:getPublicFields(req) end
