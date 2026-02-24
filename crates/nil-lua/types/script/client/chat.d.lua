-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@param req GetChatHistoryRequest
---@return GetChatHistoryResponse
function client:getChatHistory(req) end

---@param req PushChatMessageRequest
---@return ChatMessageId
function client:pushChatMessage(req) end
