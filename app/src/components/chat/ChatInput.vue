<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Button, Input } from '@tb-dev/vue-components';
import { useChatInput } from '@/composables/chat/useChatInput';

const props = defineProps<{
  onSend?: () => MaybePromise<void>;
}>();

const { t } = useI18n();

const { draft, send, prev, next } = useChatInput();

function sendMessage() {
  send({ onBeforeCommit: props.onSend });
}
</script>

<template>
  <div class="flex h-[50px] max-w-full items-center justify-between gap-2 px-1 sm:px-2 pb-2">
    <!-- Do not add a trim modifier here. -->
    <Input
      v-model="draft"
      type="text"
      :maxlength="5000"
      @keydown.enter.prevent="sendMessage"
      @keydown.up.prevent="prev"
      @keydown.down.prevent="next"
    />
    <Button :disabled="!draft" @click="sendMessage">
      {{ t('send') }}
    </Button>
  </div>
</template>
