<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { maxBy } from 'es-toolkit';
import { useI18n } from 'vue-i18n';
import { sleep } from '@tb-dev/utils';
import { handleError } from '@/lib/error';
import { localRef, useMutex } from '@tb-dev/vue';
import { Button, Input } from '@tb-dev/vue-components';
import { ChatCommand } from '@/core/model/chat/chat-command';
import { computed, nextTick, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  onSend?: () => MaybePromise<void>;
}>();

const { t } = useI18n();

const chatInput = useTemplateRef('chatInputEl');
const chatInputInner = computed(() => chatInput.value?.$el);

const draft = ref<Option<string>>();
const { locked, ...mutex } = useMutex();

interface MessageHistory {
  readonly world: WorldId;
  lastUpdate: number;
  readonly messages: {
    readonly id: number;
    readonly content: string;
  }[];
}

const history = localRef<MessageHistory[]>('chat-input:message-history', []);
const currentHistoryEntryId = ref<Option<number>>();

async function send(e: Event) {
  if (
    e instanceof KeyboardEvent &&
    e.key.toLowerCase() === 'enter' &&
    e.shiftKey
  ) {
    return;
  }

  if (draft.value) {
    try {
      await mutex.acquire();

      const message = draft.value;
      const command = new ChatCommand(message);
      await command.execute();
      await props.onSend?.();

      if (message) {
        updateHistory(message);
      }
    }
    catch (err) {
      handleError(err);
    }
    finally {
      draft.value = null;
      mutex.release();
      await nextTick();
      chatInputInner.value?.focus();
    }
  }
}

function updateHistory(message: string) {
  history.value.sort((a, b) => a.lastUpdate - b.lastUpdate);
  while (history.value.length > 10) {
    history.value.shift();
  }

  const now = Date.now();
  const world = NIL.world.getIdStrict();
  let entry = history.value.find((it) => it.world === world);

  if (!entry) {
    entry = { world, messages: [], lastUpdate: now };
    history.value.push(entry);
  }

  const id = maxBy(entry.messages, (it) => it.id)?.id;
  const nextId = (id ?? 0) + 1;

  entry.messages.push({
    id: nextId,
    content: message,
  });

  entry.lastUpdate = now;
  currentHistoryEntryId.value = null;

  entry.messages.sort((a, b) => a.id - b.id);
  while (entry.messages.length > 50) {
    entry.messages.shift();
  }
}

async function restore(direction: 'up' | 'down') {
  const world = NIL.world.getIdStrict();
  const entry = history.value.find((it) => it.world === world);
  if (entry && entry.messages.length > 0) {
    try {
      await mutex.acquire();
      switch (direction) {
        case 'up': {
          let previous: Option<MessageHistory['messages'][0]> = null;
          if (currentHistoryEntryId.value) {
            const previousId = currentHistoryEntryId.value - 1;
            previous = entry.messages.find((it) => it.id === previousId);
          }

          previous ??= entry.messages.at(-1);

          if (previous) {
            draft.value = previous.content;
            currentHistoryEntryId.value = previous.id;
          }

          break;
        }
        case 'down': {
          if (currentHistoryEntryId.value) {
            const nextId = currentHistoryEntryId.value + 1;
            const next = entry.messages.find((it) => it.id === nextId);
            if (next) {
              draft.value = next.content;
              currentHistoryEntryId.value = nextId;
            }
            else {
              draft.value = null;
              currentHistoryEntryId.value = null;
            }
          }
        }
      }
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
      if (chatInputInner.value) {
        chatInputInner.value.focus();
        await sleep(5);

        const length = chatInputInner.value.value.length;
        chatInputInner.value.setSelectionRange(length, length);
      }
    }
  }
}
</script>

<template>
  <div class="flex h-[50px] max-w-full items-center justify-between gap-2 px-1 sm:px-2 pb-2">
    <!-- Do not add a trim modifier here. -->
    <Input
      ref="chatInputEl"
      v-model="draft"
      type="text"
      :disabled="locked"
      :maxlength="5000"
      spellcheck="false"
      @keydown.enter="send"
      @keydown.up.prevent="() => restore('up')"
      @keydown.down.prevent="() => restore('down')"
    />
    <Button :disabled="!draft || locked" @click="send">
      {{ t('send') }}
    </Button>
  </div>
</template>
