-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class Maneuver
---@field id ManeuverId
---@field origin Coord
---@field destination Coord
---@field army ArmyId
---@field kind ManeuverKind
---@field direction ManeuverDirection
---@field state ManeuverState
---@field speed number
---@field hauledResources ManeuverHaul|nil

---@alias ManeuverId string

---@alias ManeuverKind 'attack' | 'support'

---@alias ManeuverDirection 'going' | 'returning'

---@alias ManeuverDistance number

---@alias ManeuverState ManeuverStateDone | ManeuverStatePending

---@class ManeuverStateDone
---@field kind 'done'

---@class ManeuverStatePending
---@field kind 'pending'
---@field distance ManeuverDistance

---@class ManeuverHaul
---@field ruler Ruler
---@field resources Resources

---@class ManeuverRequest
---@field kind ManeuverKind
---@field origin Coord
---@field destination Coord
---@field personnel ArmyPersonnel
