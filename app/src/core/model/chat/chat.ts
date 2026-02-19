// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ChatHistoryImpl } from './chat-history';
import { getChatHistory } from '@/commands/chat';

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
    this.public.push(message);
  }

  public static create(history: [ChatHistory, ChatHistory]) {
    return new ChatImpl(history);
  }

  public static async load() {
    const history = await getChatHistory();
    return ChatImpl.create([
      history.public,
      history.private,
    ]);
  }
}
