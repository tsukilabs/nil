-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class InfrastructureQueue
---@field orders InfrastructureQueueOrder[]

---@class InfrastructureQueueOrder
---@field id InfrastructureQueueOrderId
---@field resources Resources
---@field workforce number
---@field state InfrastructureQueueOrderState

---@alias InfrastructureQueueOrderId string

---@alias InfrastructureQueueOrderState
---| InfrastructureQueueOrderStatePending
---| InfrastructureQueueOrderStateDone

---@class InfrastructureQueueOrderStatePending
---@field kind "pending"
---@field workforce number

---@class InfrastructureQueueOrderStateDone
---@field kind "done"
