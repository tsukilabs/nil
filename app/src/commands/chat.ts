// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { GetChatHistoryResponse } from '@/lib/response';
import type {
  GetChatHistoryRequest,
  PushChatMessageRequest,
  PushStdioMessagesRequest,
  PushStdioMessagesRequestMessage,
} from '@/lib/request';

export async function getChatHistory() {
  const req: GetChatHistoryRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<GetChatHistoryResponse>('get_chat_history', { req });
}

export async function pushChatMessage(message: string) {
  if (message.trim().length > 0) {
    const req: PushChatMessageRequest = {
      world: NIL.world.getIdStrict(),
      message,
    };

    await invoke('push_chat_message', { req });
  }
}

export async function pushStdioMessages(messages: PushStdioMessagesRequestMessage[]) {
  if (messages.length > 0) {
    const req: PushStdioMessagesRequest = {
      world: NIL.world.getIdStrict(),
      messages,
    };

    await invoke('push_stdio_messages', { req });
  }
}
