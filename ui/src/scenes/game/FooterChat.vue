<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useToggle } from '@vueuse/core';
import Chat from '@/components/chat/Chat.vue';
import { MessageSquareTextIcon } from 'lucide-vue-next';
import { ButtonIcon, Popover, PopoverContent, PopoverTrigger } from '@tb-dev/vue-components';

const [isChatOpen, toggleChat] = useToggle(false);
const closeChat = () => void toggleChat(false);
</script>

<template>
  <Popover v-model:open="isChatOpen">
    <PopoverTrigger as-child>
      <ButtonIcon :icon="MessageSquareTextIcon" />
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
