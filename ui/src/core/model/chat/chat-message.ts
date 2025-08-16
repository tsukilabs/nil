// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { fromZoned } from '@/lib/date';
import { formatDate, isToday } from 'date-fns';

export class ChatMessageImpl implements ChatMessage {
  public readonly id: ChatMessageId;
  public readonly kind: ChatMessageKind;
  public readonly author: ChatMessageAuthor;
  public readonly content: ChatMessageContent;
  public readonly timestamp: string;
  public readonly date: Date;

  private constructor(message: ChatMessage) {
    this.id = message.id;
    this.kind = message.kind;
    this.author = message.author;
    this.content = message.content;
    this.timestamp = message.timestamp;
    this.date = fromZoned(message.timestamp);
  }

  public isDefault() {
    return this.kind === 'default';
  }

  public isStdout() {
    return this.kind === 'stdout';
  }

  public isEmpty() {
    return this.content.trim().length === 0;
  }

  public formatDate() {
    const format = isToday(this.date) ? 'HH:mm' : 'dd/MM HH:mm';
    return formatDate(this.date, format);
  }

  public static create(message: ChatMessage) {
    return new ChatMessageImpl(message);
  }
}
