// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function getChatHistory() {
  return invoke<[ChatHistory, ChatHistory]>('get_chat_history');
}

export function pushChatMessage(message: string) {
  return invoke<null>('push_chat_message', { message });
}
