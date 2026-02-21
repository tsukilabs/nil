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
const mutex = useMutex();

interface Message {
  readonly id: number;
  readonly content: string;
}

const history = localRef<Message[]>('chat-input:history', []);
const currentHistoryEntryId = ref<Option<Message['id']>>();

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
  const id = maxBy(history.value, (it) => it.id)?.id;
  const nextId = (id ?? 0) + 1;

  history.value.push({ id: nextId, content: message });

  currentHistoryEntryId.value = null;

  history.value.sort((a, b) => a.id - b.id);
  history.value.splice(0, history.value.length - 50);
}

async function restore(direction: 'up' | 'down') {
  try {
    await mutex.acquire();
    switch (direction) {
      case 'up': {
        let previous: Option<Message> = null;
        if (currentHistoryEntryId.value) {
          const previousId = currentHistoryEntryId.value - 1;
          previous = history.value.find((it) => it.id === previousId);
        }

        previous ??= history.value.at(-1);

        if (previous) {
          draft.value = previous.content;
          currentHistoryEntryId.value = previous.id;
        }

        break;
      }
      case 'down': {
        if (currentHistoryEntryId.value) {
          const nextId = currentHistoryEntryId.value + 1;
          const next = history.value.find((it) => it.id === nextId);
          if (next) {
            draft.value = next.content;
            currentHistoryEntryId.value = nextId;
            break;
          }
        }

        draft.value = null;
        currentHistoryEntryId.value = null;
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
</script>

<template>
  <div class="flex h-[50px] max-w-full items-center justify-between gap-2 px-1 sm:px-2 pb-2">
    <!-- Do not add a trim modifier here. -->
    <Input
      ref="chatInputEl"
      v-model="draft"
      type="text"
      :maxlength="5000"
      spellcheck="false"
      @keydown.enter="send"
      @keydown.up.prevent="() => restore('up')"
      @keydown.down.prevent="() => restore('down')"
    />
    <Button :disabled="!draft" @click="send">
      {{ t('send') }}
    </Button>
  </div>
</template>
