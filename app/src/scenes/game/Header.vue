<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import Round from './Round.vue';
import Resources from './Resources.vue';
import { Button, SidebarTrigger } from '@tb-dev/vue-components';
import { useBreakpoints } from '@/composables/util/useBreakpoints';

defineProps<{
  isHost: boolean;
  onStartRound: () => Promise<void>;
  onFinishTurn: () => Promise<void>;
}>();

const { city } = NIL.city.refs();

const { sm } = useBreakpoints();
</script>

<template>
  <header class="flex flex-col items-center justify-center gap-2 overflow-hidden">
    <div class="w-full flex items-center justify-between">
      <div class="max-w-3/5 flex items-center gap-2">
        <SidebarTrigger />
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
        <Round :is-host @start-round="onStartRound" @finish-turn="onFinishTurn" />
      </div>
    </div>

    <div class="w-full flex items-center justify-center">
      <Resources v-if="!sm" />
    </div>
  </header>
</template>
