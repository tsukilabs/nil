-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@alias UnitId AcademyUnitId | StableUnitId | WorkshopUnitId

---@alias AcademyUnitId 'archer' | 'axeman' | 'pikeman' | 'swordsman'

---@alias StableUnitId 'heavy-cavalry' | 'light-cavalry'

---@alias WorkshopUnitId 'ram'

---@alias UnitKind 'infantry' | 'cavalry' | 'ranged'

---@class UnitStats
---@field attack number
---@field infantryDefense number
---@field cavalryDefense number
---@field rangedDefense number
---@field rangedDebuff number
---@field speed number
---@field haul number
