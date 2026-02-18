-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class BattleResult
---@field attackerPersonnel ArmyPersonnel
---@field attackerSurvivingPersonnel ArmyPersonnel
---@field defenderPersonnel ArmyPersonnel
---@field defenderSurvivingPersonnel ArmyPersonnel
---@field wallLevel BuildingLevel
---@field winner BattleWinner
---@field luck Luck

---@alias BattleWinner 'attacker' | 'defender'

---@alias Luck number
