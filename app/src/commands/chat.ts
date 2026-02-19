// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { GetChatHistoryResponse } from '@/lib/response';
import type {
  GetChatHistoryRequest,
  PushChatMessageRequest,
  PushStdoutMessageRequest,
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

export async function pushStdoutMessage(stdout: string) {
  if (stdout.trim().length > 0) {
    const req: PushStdoutMessageRequest = {
      world: NIL.world.getIdStrict(),
      stdout,
    };

    await invoke('push_stdout_message', { req });
  }
}
