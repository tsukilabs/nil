// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { GetChatHistoryResponse } from '@/lib/response';
import type { GetChatHistoryRequest, PushChatMessageRequest } from '@/lib/request';

export async function getChatHistory() {
  const req: GetChatHistoryRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<GetChatHistoryResponse>('get_chat_history', { req });
}

export async function pushChatMessage(message: string) {
  const req: PushChatMessageRequest = {
    world: NIL.world.getIdStrict(),
    message,
  };

  return invoke<ChatMessageId>('push_chat_message', { req });
}
