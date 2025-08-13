<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Units from './Units.vue';
import Buildings from './Buildings.vue';
import Production from './Production.vue';
import { breakpointsTailwind, useBreakpoints } from '@vueuse/core';
import { useIdleArmiesAt } from '@/composables/military/useIdleArmiesAt';
import { foldArmyPersonnel } from '@/composables/military/foldArmyPersonnel';

const { coord, village } = NIL.village.refs();

const armies = useIdleArmiesAt(coord);
const personnel = foldArmyPersonnel(armies);

const { sm } = useBreakpoints(breakpointsTailwind);
</script>

<template>
  <div class="game-layout">
    <div v-if="village" class="flex size-full gap-4" :class="sm ? 'flex-row' : 'flex-col'">
      <Buildings :village class="h-min w-full" />
      <div v-if="sm" class="flex flex-col size-full max-w-80 gap-4">
        <Production class="h-min w-full" />
        <Units v-if="!personnel.isEmpty()" :personnel class="h-min w-full" />
      </div>
      <template v-else>
        <Production class="h-min w-full" />
        <Units v-if="!personnel.isEmpty()" :personnel class="h-min w-full" />
      </template>
    </div>
  </div>
</template>
