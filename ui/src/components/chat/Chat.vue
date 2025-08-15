<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import MessagePlayer from './MessagePlayer.vue';
import { ListenerSet } from '@/lib/listener-set';
import { ScrollArea } from '@tb-dev/vue-components';
import { nextTick, onMounted, useTemplateRef, type VNode } from 'vue';

interface Props {
  scrollHeight?: string;
}

withDefaults(defineProps<Props>(), {
  scrollHeight: 'calc(100% - 60px) !important',
});

defineSlots<{
  input?: (props: { scroll: typeof scroll; }) => VNode;
}>();

const { chat } = NIL.chat.refs();

const content = useTemplateRef('contentEl');

const listener = new ListenerSet();
listener.event.onChatUpdated(scroll);

async function scroll() {
  await nextTick();
  content.value?.parentElement?.parentElement?.scrollTo({
    top: Number.MAX_SAFE_INTEGER,
    behavior: 'auto',
  });
}

onMounted(scroll);

defineExpose({ scroll });
</script>

<template>
  <div class="overflow-hidden">
    <div class="flex size-full flex-col gap-4">
      <div class="flex h-full flex-col justify-between gap-4 overflow-hidden">
        <ScrollArea :style="{ height: scrollHeight }">
          <div v-if="chat" ref="contentEl" class="flex flex-col gap-3 pr-6 pl-2 sm:pl-4">
            <div v-for="message of chat" :key="message.id" class="chat-message">
              <MessagePlayer v-if="message.author.kind === 'player'" :message />
            </div>
          </div>
        </ScrollArea>

        <slot name="input" v-bind="{ scroll }"></slot>
      </div>
    </div>
  </div>
</template>
