// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ChatHistoryImpl } from './chat-history';
import { getChatHistory } from '@/commands/chat';

export class ChatImpl {
  public readonly history: ChatHistoryImpl;

  private constructor(history: ChatHistory) {
    this.history = ChatHistoryImpl.create(history);
  }

  public *[Symbol.iterator]() {
    yield* this.history;
  }

  public push(message: ChatMessage) {
    this.history.push(message);
  }

  public static create(history: ChatHistory) {
    return new ChatImpl(history);
  }

  public static async load() {
    const history = await getChatHistory();
    return ChatImpl.create(history);
  }
}
