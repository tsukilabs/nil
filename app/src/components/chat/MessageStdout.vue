<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import type { ChatMessageImpl } from '@/core/model/chat/chat-message';

const props = defineProps<{
  message: ChatMessageImpl;
}>();

const content = computed(() => {
  return props.message.content.trim().split('\n');
});
</script>

<template>
  <template v-for="(line, idx) of content" :key="idx">
    <div class="flex w-full items-center overflow-hidden gap-2 md:gap-4">
      <span class="text-sm text-yellow-600">$</span>
      <pre class="text-secondary-foreground/80 text-pretty break-normal wrap-anywhere whitespace-normal select-text">
        {{ line }}
      </pre>
    </div>
  </template>
</template>
