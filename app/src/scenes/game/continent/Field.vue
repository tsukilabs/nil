<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import FieldCity from './FieldCity.vue';
import type { PublicFieldImpl } from '@/core/model/continent/public-field';

const props = defineProps<{
  field: PublicFieldImpl;
  currentCoord: Option<Coord>;
}>();

const { continentSize } = NIL.world.refs();

const isOutside = computed(() => {
  return props.field.isOutside();
});

const classList = computed(() => {
  let cl = isOutside.value ? 'field' : 'field inside';
  if (isOutside.value) {
    cl += ' border-0';
  }
  else {
    const size = continentSize.value;
    cl += props.field.x === size - 1 ? ' border-r' : ' border-r-0';
    cl += props.field.y === size - 1 ? ' border-t' : ' border-t-0';
  }

  return cl;
});
</script>

<template>
  <div :data-x="field.x" :data-y="field.y" :class="classList">
    <div class="relative flex size-full flex-col">
      <FieldCity v-if="field.isCity()" :field :current-coord />
    </div>
  </div>
</template>

<style scoped>
.field {
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.field.inside {
  border-bottom-width: 1px;
  border-left-width: 1px;
  border-style: solid;
  border-color: var(--border);
  background-color: var(--primary-foreground);
}

.field:not(.inside) {
  background-color: color-mix(in oklab, var(--background) 40%, transparent);
}

.field:not(.inside) > :deep(div) {
  visibility: hidden;
}
</style>
