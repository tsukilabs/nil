// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { getChat } from '@/commands/chat';
import { ChatHistoryImpl } from './chat-history';

export class ChatImpl implements Chat {
  public readonly history: ChatHistoryImpl;

  private constructor(chat: Chat) {
    this.history = ChatHistoryImpl.create(chat.history);
  }

  public *[Symbol.iterator]() {
    yield* this.history.queue;
  }

  public static create(chat: Chat) {
    return new ChatImpl(chat);
  }

  public static async load() {
    const chat = await getChat();
    return ChatImpl.create(chat);
  }
}
