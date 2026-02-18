-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@param req GetChatHistoryRequest
---@return [ChatHistory, ChatHistory]
function client:getChatHistory(req) end

---@param req PushChatMessageRequest
function client:pushChatMessage(req) end
