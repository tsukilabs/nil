<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Resources from './Resources.vue';
import FooterChat from './FooterChat.vue';
import Unread from '@/components/Unread.vue';
import ButtonIcon from '@/components/ButtonIcon.vue';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import { EarthIcon, ScrollTextIcon, SwordsIcon } from 'lucide-vue-next';

const { unread: unreadReports } = NIL.report.refs();

const { sm } = useBreakpoints();
</script>

<template>
  <footer class="flex items-center justify-between gap-4 overflow-hidden">
    <Resources v-if="sm" />
    <div class="flex size-full min-w-max items-center justify-end gap-2 sm:gap-4">
      <RouterLink :to="{ name: 'war-room' satisfies GameScene }">
        <ButtonIcon :icon="SwordsIcon" />
      </RouterLink>

      <RouterLink :to="{ name: 'continent' satisfies GameScene }">
        <ButtonIcon :icon="EarthIcon" />
      </RouterLink>

      <RouterLink :to="{ name: 'report' satisfies GameScene }">
        <Unread :icon="ScrollTextIcon" :has-unread="unreadReports.size > 0" />
      </RouterLink>

      <FooterChat />
    </div>
  </footer>
</template>
