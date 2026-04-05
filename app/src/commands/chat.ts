// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { ChatHistory, ChatMessageId } from '@/types/core/chat';
import type { GetChatHistoryRequest, PushChatMessageRequest } from '@/types/request';

export async function getChatHistory() {
  const req: GetChatHistoryRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<ChatHistory>('get_chat_history', { req });
}

export async function pushChatMessage(message: string) {
  const req: PushChatMessageRequest = {
    world: NIL.world.getIdStrict(),
    message,
  };

  return invoke<ChatMessageId>('push_chat_message', { req });
}
