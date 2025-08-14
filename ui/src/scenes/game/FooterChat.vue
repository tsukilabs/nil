<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import Chat from '@/components/chat/Chat.vue';
import { ListenerSet } from '@/lib/listener-set';
import { useToggle, whenever } from '@vueuse/core';
import ChatIcon from '@/components/chat/ChatIcon.vue';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  Popover,
  PopoverContent,
  PopoverTrigger,
  VisuallyHidden,
} from '@tb-dev/vue-components';

const { t } = useI18n();

const { player } = NIL.player.refs();

const [isChatOpen, toggleChat] = useToggle(false);
const closeChat = () => void toggleChat(false);

const [hasUnread, toggleUnread] = useToggle(false);

const { sm } = useBreakpoints();

const listener = new ListenerSet();
listener.event.onChatUpdated(onChatUpdated);

whenever(isChatOpen, () => void toggleUnread(false));

function onChatUpdated({ message }: ChatUpdatedPayload) {
  if (!isChatOpen.value && message.author.id !== player.value?.id) {
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
      class="w-96 max-w-[90vw] h-[500px] max-h-[75vh]"
      @pointer-down-outside="closeChat"
    >
      <Chat class="w-full h-[470px] max-h-[calc(75vh-30px)]" />
    </PopoverContent>
  </Popover>

  <Dialog v-else v-model:open="isChatOpen">
    <DialogTrigger as-child>
      <ChatIcon :has-unread />
    </DialogTrigger>

    <DialogContent
      class="w-96 max-w-[90vw] h-[500px] max-h-[75vh] px-2"
      @pointer-down-outside="closeChat"
    >
      <VisuallyHidden>
        <DialogHeader>
          <DialogTitle>{{ t('chat') }}</DialogTitle>
          <DialogDescription>
            {{ t('chat') }}
          </DialogDescription>
        </DialogHeader>
      </VisuallyHidden>

      <Chat class="w-full h-[470px] max-h-[calc(75vh-30px)]" />
    </DialogContent>
  </Dialog>
</template>
