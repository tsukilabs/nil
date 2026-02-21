<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useBreakpoints } from '@tb-dev/vue';
import { usePlayerResources } from '@/composables/player/usePlayerResources';
import { usePlayerStorageCapacity } from '@/composables/player/usePlayerStorageCapacity';

const resources = usePlayerResources();
const capacity = usePlayerStorageCapacity();

const { sm } = useBreakpoints();

const iconClass = computed(() => !sm.value ? 'size-2 min-h-2 min-w-2' : null);
const textClass = computed(() => !sm.value ? 'text-sm' : null);
</script>

<template>
  <div class="w-max max-w-max flex items-center gap-2 pr-4">
    <Wood :amount="resources?.wood" :limit="capacity?.warehouse" :icon-class :text-class class="md:pr-8" />
    <Stone :amount="resources?.stone" :limit="capacity?.warehouse" :icon-class :text-class class="md:pr-8" />
    <Iron :amount="resources?.iron" :limit="capacity?.warehouse" :icon-class :text-class class="md:pr-8" />
    <Food :amount="resources?.food" :limit="capacity?.silo" :icon-class :text-class class="md:pr-8" />
  </div>
</template>
