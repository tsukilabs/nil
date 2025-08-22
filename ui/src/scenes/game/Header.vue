<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import Round from './Round.vue';
import { Button, SidebarTrigger } from '@tb-dev/vue-components';
import { useBreakpoints } from '@/composables/util/useBreakpoints';

defineProps<{
  isHost: boolean;
  onStartRound: () => MaybePromise<void>;
  onFinishTurn: () => MaybePromise<void>;
}>();

const { city } = NIL.city.refs();

const { sm } = useBreakpoints();
</script>

<template>
  <header class="flex items-center justify-between overflow-hidden">
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
      <Round
        :is-host
        @start-round="onStartRound"
        @finish-turn="onFinishTurn"
      />
    </div>
  </header>
</template>
