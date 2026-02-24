// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { formatToday, fromZoned } from '@/lib/date';
import { compareAsc as compareDateAsc } from 'date-fns';

export class ChatMessageImpl implements ChatMessage {
  public readonly id: ChatMessageId;
  public readonly kind: ChatMessageKind;
  public readonly author: ChatMessageAuthor;
  public readonly content: ChatMessageContent;
  public readonly time: string;
  public readonly date: Date;

  private constructor(message: ChatMessage) {
    this.id = message.id;
    this.kind = message.kind;
    this.author = message.author;
    this.content = message.content;
    this.time = message.time;
    this.date = fromZoned(message.time);
  }

  public isEmpty() {
    return this.content.trim().length === 0;
  }

  public compareDateAsc(other: ChatMessage) {
    const otherImpl = ChatMessageImpl.create(other);
    return compareDateAsc(this.date, otherImpl.date);
  }

  public formatDate() {
    return formatToday(this.date);
  }

  public static create(message: ChatMessage) {
    return new ChatMessageImpl(message);
  }
}
