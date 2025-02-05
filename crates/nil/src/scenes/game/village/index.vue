<script setup lang="ts">
import Units from './Units.vue';
import { Game } from '@/core/game';
import Buildings from './Buildings.vue';
import Production from './Production.vue';

const { village } = Game.use();
</script>

<template>
  <div class="size-full">
    <div v-if="village" class="flex size-full gap-4">
      <Buildings :village class="h-min w-full" />
      <div class="hidden size-full max-w-80 flex-col gap-4 sm:flex">
        <Production class="h-min w-full" />
        <Units class="h-min w-full" />
      </div>
    </div>
  </div>
</template>
