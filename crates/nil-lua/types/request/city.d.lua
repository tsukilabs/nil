-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class GetCityRequest
---@field world WorldId
---@field coord Coord

---@class GetCityScoreRequest
---@field world WorldId
---@field coord Coord

---@class GetPublicCitiesRequest
---@field world WorldId
---@field coords Coord[]
---@field score boolean|nil
---@field all boolean|nil

---@class GetPublicCityRequest
---@field world WorldId
---@field coord Coord
---@field score boolean|nil

---@class RenameCityRequest
---@field world WorldId
---@field coord Coord
---@field name string

---@class SearchCityRequest
---@field world WorldId
---@field search CitySearch

---@class SearchPublicCityRequest
---@field world WorldId
---@field search CitySearch
