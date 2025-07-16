<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { ref } from 'vue';
import Chat from '@/components/chat/Chat.vue';
import { ListenerSet } from '@/lib/listener-set';
import { useToggle, whenever } from '@vueuse/core';
import { MessageSquareTextIcon } from 'lucide-vue-next';
import { Button, Popover, PopoverContent, PopoverTrigger } from '@tb-dev/vue-components';

const { player } = NIL.player.refs();

const [isChatOpen, toggleChat] = useToggle(false);
const closeChat = () => void toggleChat(false);

const hasUnread = ref(false);

const listener = new ListenerSet();
listener.event.onChatUpdated(onChatUpdated);

whenever(isChatOpen, () => (hasUnread.value = false));

function onChatUpdated({ message }: ChatUpdatedPayload) {
  if (!isChatOpen.value && message.message.author !== player.value?.id) {
    hasUnread.value = true;
  }
}
</script>

<template>
  <Popover v-model:open="isChatOpen">
    <PopoverTrigger as-child>
      <div class="relative">
        <Button variant="ghost" size="icon">
          <MessageSquareTextIcon />
        </Button>

        <div
          v-if="hasUnread"
          class="absolute top-[4px] right-[4px] size-[10px] min-h-[10px] min-w-[10px] overflow-hidden rounded-full bg-red-500"
        ></div>
      </div>
    </PopoverTrigger>

    <PopoverContent
      align="end"
      :align-offset="-15"
      side="top"
      :side-offset="10"
      class="h-[500px] max-h-[75vh] w-96"
      @pointer-down-outside="closeChat"
    >
      <Chat class="h-[470px] max-h-[calc(75vh-30px)] w-90" />
    </PopoverContent>
  </Popover>
</template>
