<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import FieldVillage from './FieldVillage.vue';
import type { PublicFieldImpl } from '@/core/model/continent/public-field';

const props = defineProps<{
  field: PublicFieldImpl;
  continentSize: number;
}>();

const isOutside = computed(() => {
  return props.field.isOutside(props.continentSize);
});

const classList = computed(() => {
  return isOutside.value ? 'field' : 'field inside';
});
</script>

<template>
  <div :data-x="field.x" :data-y="field.y" :class="classList">
    <div class="bg-primary-foreground relative flex size-full flex-col">
      <FieldVillage v-if="field.isVillage() && field.village" :field :village="field.village" />
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
  border-top-width: v-bind("field.y === (continentSize - 1) ? '1px' : 0");
  border-right-width: v-bind("field.x === (continentSize - 1) ? '1px' : 0");
  border-bottom-width: 1px;
  border-left-width: 1px;
  border-style: solid;
  border-color: var(--border);
}

.field:not(.inside) > :deep(div) {
  visibility: hidden;
}
</style>
