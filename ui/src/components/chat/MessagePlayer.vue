<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { fromZoned } from '@/lib/date';
import { format as formatDate, isToday } from 'date-fns';

const props = defineProps<{ message: ChatMessagePlayer }>();

const datetime = computed(() => {
  const date = fromZoned(props.message.time);
  const format = isToday(date) ? 'HH:mm' : 'dd/MM HH:mm';
  return formatDate(date, format);
});
</script>

<template>
  <div class="flex w-full flex-col gap-1 overflow-hidden">
    <div class="text-muted-foreground flex items-center justify-between gap-4 text-xs">
      <span class="ellipsis">{{ message.author }}</span>
      <span class="hidden lg:block">{{ datetime }}</span>
    </div>
    <div class="text-pretty break-normal wrap-anywhere whitespace-normal select-text">
      <span>{{ message.content }}</span>
    </div>
  </div>
</template>
