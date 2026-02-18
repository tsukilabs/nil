-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class Army
---@field id ArmyId
---@field personnel ArmyPersonnel
---@field owner Ruler
---@field state ArmyState

---@class ArmyPersonnel
---@field archer Squad
---@field axeman Squad
---@field pikeman Squad
---@field swordsman Squad
---@field heavyCavalry Squad
---@field lightCavalry Squad
---@field ram Squad

---@class ArmyAcademyPersonnel
---@field archer Squad
---@field axeman Squad
---@field pikeman Squad
---@field swordsman Squad

---@class ArmyStablePersonnel
---@field heavyCavalry Squad
---@field lightCavalry Squad

---@class ArmyWorkshopPersonnel
---@field ram Squad

---@alias ArmyId string

---@alias ArmyState ArmyStateIdle | ArmyStateManeuvering

---@class ArmyStateIdle
---@field kind 'idle'

---@class ArmyStateManeuvering
---@field kind 'maneuvering'
---@field maneuver ManeuverId

---@class ArmyPersonnelSize
---@field archer number
---@field axeman number
---@field pikeman number
---@field swordsman number
---@field heavyCavalry number
---@field lightCavalry number
---@field ram number
