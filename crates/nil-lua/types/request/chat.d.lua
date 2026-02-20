-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class GetChatHistoryRequest
---@field world WorldId

---@class PushChatMessageRequest
---@field world WorldId
---@field message string

---@class PushStdioMessagesRequest
---@field world WorldId
---@field messages PushStdioMessagesRequestMessage[]

---@class PushStdioMessagesRequestMessage
---@field content string
---@field time string|nil
