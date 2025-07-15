<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import type { Option } from '@tb-dev/utils';
import type { HighlighterCore } from 'shiki/core';
import type { ScriptContents } from '@/composables/script/useNsr';

const props = defineProps<{
  contents: Option<ScriptContents>;
  highlighter: HighlighterCore;
}>();

const code = computed(() => {
  const script = props.contents?.script;
  return script ? toHtml(script) : null;
});

function toHtml(raw: string) {
  return props.highlighter.codeToHtml(raw, {
    lang: 'lua',
    theme: 'vesper',
  });
}
</script>

<template>
  <div class="flex size-full flex-col gap-4 p-4 select-text">
    <div v-if="contents" v-html="contents.about"></div>
    <div v-if="code" v-html="code"></div>
  </div>
</template>

<style scoped>
:deep(h1, h2, h3, h4, h5, h6) {
  margin-bottom: 1rem;
  font-weight: 400;
  font-size: 1.5rem;
  line-height: 24px;
}

:deep(h1) {
  margin-bottom: 1rem;
  font-size: 1.5rem;
}

:deep(h1, h2, h3, h4, h5, h6, li, p) {
  overflow-wrap: break-word;
}

:deep(pre) {
  border-radius: var(--radius);
  padding: 1rem;
}
</style>
