<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Round from './Round.vue';
import type { MaybePromise } from '@tb-dev/utils';
import { Button, SidebarTrigger } from '@tb-dev/vue-components';

defineProps<{
  isHost: boolean;
  onStartRound: () => MaybePromise<void>;
  onEndTurn: () => MaybePromise<void>;
}>();

const { village } = NIL.village.refs();
</script>

<template>
  <header class="flex items-center justify-between">
    <div class="flex items-center gap-2">
      <SidebarTrigger />
      <Button v-if="village" variant="ghost" class="py-2 text-base lg:text-lg">
        <RouterLink :to="{ name: 'village' satisfies GameScene }" class="space-x-1">
          <span>{{ village.name }}</span>
          <span>({{ village.coord.format() }})</span>
        </RouterLink>
      </Button>
    </div>

    <div class="flex items-center">
      <Round :is-host @start-round="onStartRound" @end-turn="onEndTurn" />
    </div>
  </header>
</template>
