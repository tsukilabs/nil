// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ChatMessageImpl } from './chat-message';

export class ChatHistoryImpl implements ChatHistory {
  public readonly queue: ChatMessageImpl[];
  public readonly size: number;

  private constructor(history: ChatHistory) {
    this.queue = history.queue.map((it) => ChatMessageImpl.create(it));
    this.size = history.size;

    this.sort();
  }

  public *[Symbol.iterator]() {
    yield* this.queue;
  }

  public first() {
    return this.queue.at(0);
  }

  public last() {
    return this.queue.at(-1);
  }

  public push(message: ChatMessage) {
    if (this.queue.every(({ id }) => id !== message.id)) {
      this.queue.push(ChatMessageImpl.create(message));
      this.sort();
    }
  }

  public sort() {
    this.queue.sort((a, b) => a.date.getTime() - b.date.getTime());
  }

  public static create(history: ChatHistory) {
    return new ChatHistoryImpl(history);
  }
}
