-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class WorldOptions
---@field name string
---@field size number
---@field locale Locale
---@field allowCheats boolean
---@field speed WorldSpeed
---@field unitSpeed WorldUnitSpeed
---@field botDensity BotDensity
---@field botAdvancedStartRatio BotAdvancedStartRatio

---@class WorldConfig
---@field id WorldId
---@field name string
---@field locale Locale
---@field allowCheats boolean
---@field speed WorldSpeed
---@field unitSpeed WorldUnitSpeed
---@field botDensity BotDensity
---@field botAdvancedStartRatio BotAdvancedStartRatio

---@alias WorldId string

---@alias Locale 'en-US' | 'pt-BR'

---@alias WorldSpeed number

---@alias WorldUnitSpeed number

---@alias BotDensity number

---@alias BotAdvancedStartRatio number

---@alias WorldStats table
