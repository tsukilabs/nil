-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class Round
---@field id RoundId
---@field state RoundState

---@alias RoundId number

---@alias RoundState RoundStateIdle | RoundStateWaiting | RoundStateDone

---@class RoundStateIdle
---@field kind 'idle'

---@class RoundStateWaiting
---@field kind 'waiting'
---@field pending PlayerId[]
---@field ready PlayerId[]

---@class RoundStateDone
---@field kind 'done'
