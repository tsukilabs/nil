// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from './abstract';
import { compare } from '@/lib/intl';
import { asyncRef } from '@tb-dev/vue';
import { getChatMessages } from '@/commands';
import { type ShallowRef, triggerRef } from 'vue';

export class ChatEntity extends Entity {
  private readonly chat: ShallowRef<ChatMessage[]>;

  private readonly updateChat: () => Promise<void>;

  constructor() {
    super();

    const chat = asyncRef([], getChatMessages);
    this.chat = chat.state;
    this.updateChat = chat.execute;

    this.initListeners();
  }

  protected override initListeners() {
    this.event.onChatMessage(this.onChatMessage.bind(this));
  }

  public override async update() {
    await this.updateChat();
  }

  private onChatMessage({ message }: ChatMessagePayload) {
    this.chat.value.push(message);
    this.chat.value.sort((a, b) => compare(a.message.id, b.message.id));
    triggerRef(this.chat);
  }

  public static use() {
    return super.get(ChatEntity) as ChatEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      chat: instance.chat as Readonly<ShallowRef<readonly ChatMessage[]>>,
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
        value: chat,
        writable: false,
      });
    }
  }
}
