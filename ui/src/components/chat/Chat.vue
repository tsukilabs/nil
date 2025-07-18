<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { onKeyDown } from '@tb-dev/vue';
import { handleError } from '@/lib/error';
import type { Option } from '@tb-dev/utils';
import { pushChatMessage } from '@/commands';
import MessagePlayer from './MessagePlayer.vue';
import { Button, Input, ScrollArea } from '@tb-dev/vue-components';
import { computed, nextTick, onMounted, ref, useTemplateRef } from 'vue';

const { t } = useI18n();

const { chat } = NIL.chat.refs();

const content = useTemplateRef('contentEl');
const chatInput = useTemplateRef('chatInputEl');
const chatInputInner = computed(() => chatInput.value?.$el);

const draft = ref<Option<string>>();
const loading = ref(false);

onKeyDown('Enter', send, { target: chatInputInner });

async function send() {
  if (draft.value && !loading.value) {
    try {
      loading.value = true;
      const id = await pushChatMessage(draft.value);
      draft.value = null;
      void scroll(id);
    } catch (err) {
      handleError(err);
    } finally {
      loading.value = false;
      await nextTick();
      chatInputInner.value?.focus();
    }
  }
}

async function scroll(id: ChatMessageId) {
  const elementId = `#${toElementId(id)}`;
  await content.value?.waitScroll(elementId, {
    behavior: 'instant',
    timeout: 1000,
    throwOnTimeout: false,
  });
}

function toElementId(id: ChatMessageId) {
  return `chat-message-${id}`;
}

onMounted(() => {
  const last = chat.value.at(-1);
  if (last) {
    void scroll(last.message.id);
  }
});
</script>

<template>
  <div class="overflow-hidden">
    <div class="flex size-full flex-col gap-4">
      <div class="flex h-full flex-col gap-4 overflow-hidden">
        <ScrollArea class="h-[calc(100%-50px)]">
          <div ref="contentEl" class="flex flex-col gap-3 pr-6 pl-4">
            <div v-for="{ kind, message } of chat" :id="toElementId(message.id)" :key="message.id">
              <MessagePlayer v-if="kind === 'player'" :message />
            </div>
          </div>
        </ScrollArea>

        <div class="flex h-[30px] items-center justify-between gap-2 px-2 pb-2">
          <Input
            ref="chatInputEl"
            v-model.trim="draft"
            type="text"
            :disabled="loading"
            :maxlength="200"
          />
          <Button :disabled="!draft || loading" @click="send">{{ t('send') }}</Button>
        </div>
      </div>
    </div>
  </div>
</template>
