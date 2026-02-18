-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@param req GetCityRequest
---@return City
function client:getCity(req) end

---@param req GetCityScoreRequest
---@return Score
function client:getCityScore(req) end

---@param req GetPublicCitiesRequest
---@return GetPublicCityResponse[]
function client:getPublicCities(req) end

---@param req GetPublicCityRequest
---@return GetPublicCityResponse
function client:getPublicCity(req) end

---@param req RenameCityRequest
function client:renameCity(req) end

---@param req SearchCityRequest
---@return City[]
function client:searchCity(req) end

---@param req SearchPublicCityRequest
---@return PublicCity[]
function client:searchPublicCity(req) end
