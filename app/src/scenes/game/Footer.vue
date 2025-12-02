<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Resources from './Resources.vue';
import FooterChat from './FooterChat.vue';
import { useBreakpoints } from '@/composables/util/useBreakpoints';

const { sm } = useBreakpoints();
</script>

<template>
  <footer class="flex items-center justify-between gap-4 overflow-hidden">
    <Resources v-if="sm" />
    <div class="flex size-full min-w-max items-center justify-end">
      <FooterChat />
    </div>
  </footer>
</template>
