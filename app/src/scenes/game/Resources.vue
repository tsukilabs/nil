<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { usePlayerResources } from '@/composables/player/usePlayerResources';
import { usePlayerStorageCapacity } from '@/composables/player/usePlayerStorageCapacity';

const resources = usePlayerResources();
const capacity = usePlayerStorageCapacity();
</script>

<template>
  <div class="w-max max-w-max grid grid-cols-4 items-center gap-2 md:gap-4 xl:gap-8 pr-4">
    <Wood :amount="resources?.wood" :limit="capacity?.warehouse" />
    <Stone :amount="resources?.stone" :limit="capacity?.warehouse" />
    <Iron :amount="resources?.iron" :limit="capacity?.warehouse" />
    <Food :amount="resources?.food" :limit="capacity?.silo" />
  </div>
</template>
