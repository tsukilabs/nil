<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { handleError } from '@/lib/error';
import { pushChatMessage } from '@/commands';
import MessagePlayer from './MessagePlayer.vue';
import { ListenerSet } from '@/lib/listener-set';
import { onKeyDown, useMutex } from '@tb-dev/vue';
import { Button, Input, ScrollArea } from '@tb-dev/vue-components';
import { computed, nextTick, onMounted, ref, useTemplateRef } from 'vue';

const { t } = useI18n();

const { chat } = NIL.chat.refs();

const content = useTemplateRef('contentEl');
const chatInput = useTemplateRef('chatInputEl');
const chatInputInner = computed(() => chatInput.value?.$el);

const draft = ref<Option<string>>();
const { locked, ...mutex } = useMutex();

const listener = new ListenerSet();
listener.event.onChatUpdated(scroll);

onKeyDown('Enter', send, { target: chatInputInner });

async function send() {
  if (draft.value) {
    try {
      await mutex.acquire();
      await pushChatMessage(draft.value);
      draft.value = null;
      await scroll();
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
      await nextTick();
      chatInputInner.value?.focus();
    }
  }
}

async function scroll() {
  await nextTick();
  await content.value?.waitChild('.chat-message', 500);
  content.value?.parentElement?.parentElement?.scrollTo({
    top: Number.MAX_SAFE_INTEGER,
    behavior: 'auto',
  });
}

onMounted(scroll);
</script>

<template>
  <div class="overflow-hidden">
    <div class="flex size-full flex-col gap-4">
      <div class="flex h-full flex-col justify-between gap-4 overflow-hidden">
        <ScrollArea class="h-[calc(100%-60px)]!">
          <div v-if="chat" ref="contentEl" class="flex flex-col gap-3 pr-6 pl-2 sm:pl-4">
            <div v-for="message of chat" :key="message.id" class="chat-message">
              <MessagePlayer v-if="message.author.kind === 'player'" :message />
            </div>
          </div>
        </ScrollArea>

        <div class="flex h-[50px] max-w-full items-center justify-between gap-2 px-1 sm:px-2 pb-2">
          <Input
            ref="chatInputEl"
            v-model.trim="draft"
            type="text"
            :disabled="locked"
            :maxlength="200"
          />
          <Button :disabled="!draft || locked" @click="send">{{ t('send') }}</Button>
        </div>
      </div>
    </div>
  </div>
</template>
