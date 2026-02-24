<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import Round from './Round.vue';
import Resources from './Resources.vue';
import { useBreakpoints } from '@tb-dev/vue';
import { Button, Separator, SidebarTrigger } from '@tb-dev/vue-components';

defineProps<{
  isHost: boolean;
  isPlayerReady: boolean;
  isPlayerTurn: boolean;
  isRemoteCreatedBySelf: boolean;
  onStartRound: () => Promise<void>;
  onTogglePlayerReady: () => Promise<void>;
}>();

const { city } = NIL.city.refs();

const { sm } = useBreakpoints();
</script>

<template>
  <header class="flex flex-col items-center justify-center gap-2 overflow-hidden">
    <div class="w-full flex items-center justify-between">
      <div class="max-w-3/5 flex items-center shrink-0 gap-2">
        <SidebarTrigger class="-ml-1" />
        <Separator orientation="vertical" class="data-[orientation=vertical]:h-4" />
        <Button
          v-if="city"
          variant="ghost"
          role="link"
          tabindex="0"
          class="py-2 text-base lg:text-lg"
          @click="() => go('city')"
        >
          <div class="space-x-1">
            <template v-if="sm">
              <span>{{ city.name }}</span>
              <span>({{ city.coord.format() }})</span>
            </template>
            <span v-else>{{ city.coord.format() }}</span>
          </div>
        </Button>
      </div>

      <div class="flex items-center">
        <Round
          :is-host
          :is-player-ready
          :is-player-turn
          :is-remote-created-by-self
          @start-round="onStartRound"
          @toggle-player-ready="onTogglePlayerReady"
        />
      </div>
    </div>

    <div class="w-full flex items-center justify-center">
      <Resources v-if="!sm" />
    </div>
  </header>
</template>
