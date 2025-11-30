<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { watchEffect } from 'vue';
import Chat from '@/components/chat/Chat.vue';
import { useRoute, useRouter } from 'vue-router';
import { ListenerSet } from '@/lib/listener-set';
import { useToggle, whenever } from '@vueuse/core';
import ChatIcon from '@/components/chat/ChatIcon.vue';
import ChatInput from '@/components/chat/ChatInput.vue';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import { Button, Popover, PopoverContent, PopoverTrigger } from '@tb-dev/vue-components';

const { player } = NIL.player.refs();

const [isChatOpen, toggleChat] = useToggle(false);
const [hasUnread, toggleUnread] = useToggle(false);

const route = useRoute();
const router = useRouter();

const listener = new ListenerSet();
listener.event.onChatUpdated(onChatUpdated);

const { sm } = useBreakpoints();

whenever(isChatOpen, () => void toggleUnread(false));

watchEffect(() => {
  if (route.name === ('chat' satisfies GameScene)) {
    hasUnread.value = false;
  }
});

function onChatUpdated({ message }: ChatUpdatedPayload) {
  if (
    !isChatOpen.value &&
    route.name !== ('chat' satisfies GameScene) &&
    (message.author.kind !== 'player' || message.author.id !== player.value?.id)
  ) {
    hasUnread.value = true;
  }
}
</script>

<template>
  <Popover v-if="sm" v-model:open="isChatOpen">
    <PopoverTrigger as-child>
      <ChatIcon :has-unread />
    </PopoverTrigger>

    <PopoverContent
      align="end"
      :align-offset="-15"
      side="top"
      :side-offset="10"
      disable-outside-pointer-events
      class="w-96 2xl:w-120 max-w-[90vw] h-[500px] max-h-[75vh] px-0"
      @pointer-down-outside="() => void toggleChat(false)"
    >
      <Chat class="w-full h-[470px] max-h-[calc(75vh-30px)]">
        <template #input="{ scroll }">
          <ChatInput @send="scroll" />
        </template>
      </Chat>
    </PopoverContent>
  </Popover>

  <Button
    v-else-if="route.name === ('chat' satisfies GameScene)"
    variant="ghost"
    size="icon"
    @click="() => router.back()"
  >
    <ChatIcon :has-unread />
  </Button>

  <RouterLink v-else :to="{ name: 'chat' satisfies GameScene }">
    <ChatIcon :has-unread />
  </RouterLink>
</template>
