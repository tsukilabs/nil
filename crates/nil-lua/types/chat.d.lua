-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class ChatHistory
---@field queue ChatMessage[]
---@field size number

---@class ChatMessage
---@field id ChatMessageId
---@field kind ChatMessageKind
---@field author ChatMessageAuthor
---@field content ChatMessageContent
---@field time string

---@alias ChatMessageKind 'default'

---@alias ChatMessageAuthor ChatMessageAuthorPlayer | ChatMessageAuthorSystem

---@class ChatMessageAuthorSystem
---@field kind 'system'

---@class ChatMessageAuthorPlayer
---@field kind 'player'
---@field id PlayerId

---@alias ChatMessageId string
---@alias ChatMessageContent string
