<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { handleError } from '@/lib/error';
import { onKeyDown, useMutex } from '@tb-dev/vue';
import { pushChatMessage } from '@/commands/chat';
import { Button, Input } from '@tb-dev/vue-components';
import { computed, nextTick, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  onSend?: () => MaybePromise<void>;
}>();

const { t } = useI18n();

const chatInput = useTemplateRef('chatInputEl');
const chatInputInner = computed(() => chatInput.value?.$el);

const draft = ref<Option<string>>();
const { locked, ...mutex } = useMutex();

onKeyDown('Enter', send, { target: chatInputInner });

async function send() {
  if (draft.value) {
    try {
      await mutex.acquire();
      await pushChatMessage(draft.value);
      draft.value = null;
      await props.onSend?.();
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
    <Input
      ref="chatInputEl"
      v-model.trim="draft"
      type="text"
      :disabled="locked"
      :maxlength="200"
    />
    <Button :disabled="!draft || locked" @click="send">
      {{ t('send') }}
    </Button>
  </div>
</template>
