<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useHeight } from '@tb-dev/vue';
import { toPixel } from '@tb-dev/utils';
import Chat from '@/components/chat/Chat.vue';
import type { MaybeElement } from '@vueuse/core';
import ChatInput from '@/components/chat/ChatInput.vue';
import { computed, onMounted, shallowRef, useTemplateRef } from 'vue';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@tb-dev/vue-components';

const { t } = useI18n();

const chat = useTemplateRef('chatEl');

const card = shallowRef<MaybeElement>();
const cardHeight = useHeight(card);

const header = shallowRef<MaybeElement>();
const headerHeight = useHeight(header);

const footer = shallowRef<MaybeElement>();
const footerHeight = useHeight(footer);

const contentHeight = computed(() => {
  return cardHeight.value - headerHeight.value - footerHeight.value;
});

onMounted(() => chat.value?.scroll());
</script>

<template>
  <div class="card-layout py-2!">
    <Card ref="card" class="h-full">
      <CardHeader ref="header">
        <CardTitle>{{ t('chat') }}</CardTitle>
      </CardHeader>

      <CardContent :style="{ height: toPixel(Math.max(contentHeight - 60, 0)) }">
        <Chat ref="chatEl" scroll-height="100%" class="size-full" />
      </CardContent>

      <CardFooter ref="footer" class="p-0!">
        <ChatInput class="w-full" @send="() => chat?.scroll()" />
      </CardFooter>
    </Card>
  </div>
</template>
