<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import type { ScriptImpl } from '@/core/model/scripts/script';

defineProps<{
  script: ScriptImpl;
}>();
</script>

<template>
  <div class="size-full overflow-x-hidden overflow-y-auto p-4 select-text">
    <div v-html="script.toHtml()"></div>
  </div>
</template>
