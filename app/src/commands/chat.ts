// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function getChatHistory() {
  return invoke<[ChatHistory, ChatHistory]>('get_chat_history');
}

export async function pushChatMessage(message: string) {
  if (message.trim().length > 0) {
    await invoke<null>('push_chat_message', { message });
  }
}
