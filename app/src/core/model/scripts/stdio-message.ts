// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { fromZoned } from '@/lib/date';
import { compareAsc as compareDateAsc } from 'date-fns';

export class StdioMessageImpl implements StdioMessage {
  public readonly id: number;
  public readonly content: string;
  public readonly time: string;

  #date: Option<Date> = null;

  private constructor(message: StdioMessage) {
    this.id = message.id;
    this.content = message.content;
    this.time = message.time;
  }

  get date() {
    this.#date ??= fromZoned(this.time);
    return this.#date;
  }

  public compareDateAsc(other: StdioMessage) {
    const otherImpl = StdioMessageImpl.create(other);
    const result = compareDateAsc(this.date, otherImpl.date);
    return result === 0 ? this.id - otherImpl.id : result;
  }

  public static create(message: StdioMessage) {
    if (message instanceof StdioMessageImpl) {
      return message;
    }

    return new StdioMessageImpl(message);
  }

  public static compareDateAsc(a: StdioMessage, b: StdioMessage) {
    return this.create(a).compareDateAsc(b);
  }
}
