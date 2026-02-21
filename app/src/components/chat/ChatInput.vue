<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useMutex } from '@tb-dev/vue';
import { handleError } from '@/lib/error';
import { Button, Input } from '@tb-dev/vue-components';
import { computed, nextTick, useTemplateRef } from 'vue';
import { ChatCommand } from '@/core/model/chat/chat-command';
import { useChatInput } from '@/composables/chat/useChatInput';

const props = defineProps<{
  onSend?: () => MaybePromise<void>;
}>();

const { t } = useI18n();

const chatInput = useTemplateRef('chatInputEl');
const chatInputInner = computed(() => chatInput.value?.$el);

const { draft, commit, prev, next } = useChatInput();

const mutex = useMutex();

async function send() {
  if (draft.value) {
    // We need to copy the message BEFORE trying to acquire the mutex.
    const message = draft.value;

    try {
      await mutex.acquire();
      const command = new ChatCommand(message);
      await command.execute();
      await props.onSend?.();
      commit(message);
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
</script>

<template>
  <div class="flex h-[50px] max-w-full items-center justify-between gap-2 px-1 sm:px-2 pb-2">
    <!-- Do not add a trim modifier here. -->
    <Input
      ref="chatInputEl"
      v-model="draft"
      type="text"
      :maxlength="5000"
      @keydown.enter.prevent="send"
      @keydown.up.prevent="prev"
      @keydown.down.prevent="next"
    />
    <Button :disabled="!draft" @click="send">
      {{ t('send') }}
    </Button>
  </div>
</template>
