// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { getChatHistory } from '@/commands/chat';
import { ChatHistoryImpl } from './chat-history';

export class ChatImpl {
  public readonly public: ChatHistoryImpl;
  public readonly private: ChatHistoryImpl;

  private constructor(history: [ChatHistory, ChatHistory]) {
    this.public = ChatHistoryImpl.create(history[0]);
    this.private = ChatHistoryImpl.create(history[1]);
  }

  public *[Symbol.iterator]() {
    yield* this.public.merge(this.private);
  }

  public push(message: ChatMessage) {
    switch (message.kind) {
      case 'default': {
        this.public.push(message);
        break;
      }
      case 'stdout': {
        this.private.push(message);
        break;
      }
    }
  }

  public static create(history: [ChatHistory, ChatHistory]) {
    return new ChatImpl(history);
  }

  public static async load() {
    const history = await getChatHistory();
    return ChatImpl.create(history);
  }
}
