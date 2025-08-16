<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useHeight } from '@tb-dev/vue';
import { toPixel } from '@tb-dev/utils';
import Chat from '@/components/chat/Chat.vue';
import type { MaybeElement } from '@vueuse/core';
import ChatInput from '@/components/chat/ChatInput.vue';
import { computed, onMounted, shallowRef, useTemplateRef } from 'vue';
import { Card, CardContent, CardFooter } from '@tb-dev/vue-components';

const chat = useTemplateRef('chatEl');

const card = shallowRef<MaybeElement>();
const cardHeight = useHeight(card);

const footer = shallowRef<MaybeElement>();
const footerHeight = useHeight(footer);

const contentHeight = computed(() => {
  return cardHeight.value - footerHeight.value;
});

onMounted(() => chat.value?.scroll());
</script>

<template>
  <div class="card-layout py-2!">
    <Card ref="card" class="h-full">
      <CardContent
        :style="{ height: toPixel(Math.max(contentHeight - 40, 0)) }"
        class="px-0! pt-2!"
      >
        <Chat ref="chatEl" scroll-height="100%" class="size-full" />
      </CardContent>

      <CardFooter ref="footer" class="p-0!">
        <ChatInput class="w-full" @send="() => chat?.scroll()" />
      </CardFooter>
    </Card>
  </div>
</template>
