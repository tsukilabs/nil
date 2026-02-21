// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { handleError } from '@/lib/error';
import { type InjectionKey, markRaw, ref } from 'vue';
import { ChatCommand } from '@/core/model/chat/chat-command';
import { sessionRef, tryInjectOrElse, useMutex } from '@tb-dev/vue';

const chatInputKey = Symbol() as InjectionKey<ReturnType<typeof create>>;

export function useChatInput() {
  return tryInjectOrElse(chatInputKey, create);
}

export interface EnqueueOptions {
  readonly onBeforeCommit?: () => MaybePromise<void>;
}

class MessageQueue {
  private readonly queue: string[] = [];
  private readonly mutex = useMutex();

  constructor(
    private readonly commit: (message: string) => void,
  ) {}

  public enqueue(message: string, options?: EnqueueOptions) {
    if (message.length > 0) {
      this.queue.push(message);
      void this.process(options);
    }
  }

  private async process(options?: EnqueueOptions) {
    try {
      await this.mutex.acquire();
      const head = this.queue.shift();
      if (head) {
        const command = new ChatCommand(head);
        await command.execute();
        await options?.onBeforeCommit?.();
        this.commit(head);
      }
    }
    catch (err) {
      handleError(err);
    }
    finally {
      this.mutex.release();
    }
  }
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

  const queue = new MessageQueue(commit);

  function send(options?: EnqueueOptions) {
    if (draft.value) {
      queue.enqueue(draft.value, options);
    }
  }

  function commit(value: string) {
    draft.value = null;
    currentId.value = null;
    history.value.push(markRaw(new Message(value)));
    history.value.sort((a, b) => a.id - b.id);
    history.value.splice(0, history.value.length - 50);
  }

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

  return {
    draft,
    send,
    prev,
    next,
  };
}
