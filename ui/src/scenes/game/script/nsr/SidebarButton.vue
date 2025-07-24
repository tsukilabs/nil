<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
defineProps<{
  entry: NsrScript;
  onEntryClick: (script: NsrScript) => MaybePromise<void>;
}>();
</script>

<template>
  <div
    role="button"
    class="hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50 flex w-full cursor-pointer flex-col justify-start space-y-1 rounded-md px-2 py-3"
    @click="() => onEntryClick(entry)"
  >
    <span class="ellipsis text-sm leading-none font-medium">
      {{ entry.frontmatter.name }}
    </span>
    <span class="ellipsis text-muted-foreground text-xs leading-snug font-normal">
      {{ entry.frontmatter.description }}
    </span>
  </div>
</template>
