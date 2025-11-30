<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useMutex } from '@tb-dev/vue';
import { handleError } from '@/lib/error';
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

async function send() {
  if (draft.value) {
    try {
      await mutex.acquire();
      const command = new ChatCommand(draft.value);
      await command.execute();
      await props.onSend?.();
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
    />
    <Button :disabled="!draft || locked" @click="send">
      {{ t('send') }}
    </Button>
  </div>
</template>
