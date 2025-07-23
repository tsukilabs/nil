// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function getChat() {
  return invoke<Chat>('get_chat');
}

export function pushChatMessage(message: string) {
  return invoke<ChatMessageId>('push_chat_message', { message });
}
