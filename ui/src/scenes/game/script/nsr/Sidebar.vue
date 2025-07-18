<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useTemplateRef } from 'vue';
import { useHeight } from '@tb-dev/vue';
import SidebarButton from './SidebarButton.vue';
import { ScrollArea } from '@tb-dev/vue-components';
import { type MaybePromise, toPixel } from '@tb-dev/utils';

defineProps<{
  registry: readonly NsrScript[];
  onEntryClick: (script: NsrScript) => MaybePromise<void>;
}>();

const container = useTemplateRef('containerEl');
const containerHeight = useHeight(container);

function toElementId(id: string) {
  return `nsr-${id}`;
}
</script>

<template>
  <div
    ref="containerEl"
    class="flex h-full w-60 min-w-60 flex-col justify-between overflow-hidden py-4"
  >
    <div class="size-full">
      <ScrollArea :style="{ height: toPixel(containerHeight) }">
        <div
          v-for="entry of registry"
          :id="toElementId(entry.id)"
          :key="entry.id"
          class="w-full px-2"
        >
          <SidebarButton :entry @entry-click="onEntryClick" />
        </div>
      </ScrollArea>
    </div>
  </div>
</template>
