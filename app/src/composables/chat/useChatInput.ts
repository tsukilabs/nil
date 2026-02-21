// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { type InjectionKey, markRaw, ref } from 'vue';
import { sessionRef, tryInjectOrElse } from '@tb-dev/vue';

const chatInputKey = Symbol() as InjectionKey<ReturnType<typeof create>>;

export function useChatInput() {
  return tryInjectOrElse(chatInputKey, create);
}

class Message {
  private static id = 0;

  public readonly id: number;
  public readonly content: string;

  constructor(content: string) {
    this.id = ++Message.id;
    this.content = content;
  }
}

function create() {
  const draft = ref<Option<string>>();
  const currentId = ref<Option<Message['id']>>();
  const history = sessionRef<Message[]>('chat-input:history', [], {
    deep: true,
    listenToStorageChanges: true,
    shallow: false,
  });

  function prev() {
    let prevMessage: Option<Message> = null;
    if (currentId.value) {
      const previousId = currentId.value - 1;
      prevMessage = history.value.find((it) => it.id === previousId);
    }

    prevMessage ??= history.value.at(-1);

    if (prevMessage) {
      draft.value = prevMessage.content;
      currentId.value = prevMessage.id;
    }
  }

  function next() {
    if (currentId.value) {
      const nextId = currentId.value + 1;
      const nextMessage = history.value.find((it) => it.id === nextId);
      if (nextMessage) {
        draft.value = nextMessage.content;
        currentId.value = nextId;
        return;
      }
    }

    draft.value = null;
    currentId.value = null;
  }

  function commit(value: string) {
    draft.value = null;
    currentId.value = null;
    history.value.push(markRaw(new Message(value)));
    history.value.sort((a, b) => a.id - b.id);
    history.value.splice(0, history.value.length - 50);
  }

  return {
    draft,
    prev,
    next,
    commit,
  };
}
