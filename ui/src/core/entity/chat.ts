// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from './abstract';
import { asyncRef } from '@tb-dev/vue';
import type { Option } from '@tb-dev/utils';
import { type ShallowRef, triggerRef } from 'vue';
import { ChatImpl } from '@/core/model/chat/chat';

export class ChatEntity extends Entity {
  private readonly chat: ShallowRef<Option<ChatImpl>>;

  public readonly updateChat: () => Promise<void>;

  constructor() {
    super();

    const chat = asyncRef(null, ChatImpl.load.bind(ChatImpl));
    this.chat = chat.state;
    this.updateChat = chat.execute;

    this.initListeners();
  }

  protected override initListeners() {
    this.event.onChatUpdated(this.onChatUpdated.bind(this));
  }

  public override async update() {
    await this.updateChat();
  }

  private onChatUpdated({ message }: ChatUpdatedPayload) {
    if (this.chat.value) {
      this.chat.value.history.push(message);
      triggerRef(this.chat);
    }
  }

  public static use() {
    return super.get(ChatEntity) as ChatEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      chat: instance.chat as Readonly<typeof instance.chat>,
    } as const;
  }

  public static update() {
    return this.use().update();
  }

  public static init() {
    if (!Object.hasOwn(window.NIL, 'chat')) {
      const chat: (typeof window.NIL)['chat'] = {
        refs: ChatEntity.refs.bind(ChatEntity),
        update: ChatEntity.update.bind(ChatEntity),
        use: ChatEntity.use.bind(ChatEntity),
      };

      Object.defineProperty(window.NIL, 'chat', {
        configurable: false,
        enumerable: true,
        writable: false,
        value: chat,
      });
    }
  }
}
