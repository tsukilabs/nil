-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class Report
---@field id ReportId
---@field round RoundId
---@field time string

---@alias ReportId string

---@alias ReportKind ReportKindBattle | ReportKindSupport

---@class ReportKindBattle
---@field kind 'battle'
---@field report BattleReport

---@class ReportKindSupport
---@field kind 'support'
---@field report SupportReport

---@class BattleReport: Report
---@field attacker Ruler
---@field defender Ruler
---@field origin Coord
---@field destination Coord
---@field result BattleResult
---@field hauledResources Resources

---@class SupportReport: Report
---@field sender Ruler
---@field receiver Ruler
---@field origin Coord
---@field destination Coord
---@field personnel ArmyPersonnel
