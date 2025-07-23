// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { fromZoned } from '@/lib/date';

export class ChatMessageImpl implements ChatMessage {
  public readonly id: ChatMessageId;
  public readonly author: ChatMessageAuthor;
  public readonly content: ChatMessageContent;
  public readonly timestamp: string;
  public readonly date: Date;

  private constructor(message: ChatMessage) {
    this.id = message.id;
    this.author = message.author;
    this.content = message.content;
    this.timestamp = message.timestamp;
    this.date = fromZoned(message.timestamp);
  }

  public static create(message: ChatMessage) {
    return new ChatMessageImpl(message);
  }
}
