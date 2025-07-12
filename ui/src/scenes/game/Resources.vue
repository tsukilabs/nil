<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { usePlayerResources } from '@/composables/usePlayerResources';
import { usePlayerStorageCapacity } from '@/composables/usePlayerStorageCapacity';

const resources = usePlayerResources();
const capacity = usePlayerStorageCapacity();
</script>

<template>
  <div class="grid grid-cols-4 items-center gap-8">
    <Wood :amount="resources?.wood" :limit="capacity?.warehouse" />
    <Stone :amount="resources?.stone" :limit="capacity?.warehouse" />
    <Iron :amount="resources?.iron" :limit="capacity?.warehouse" />
    <Food :amount="resources?.food" :limit="capacity?.silo" />
  </div>
</template>
