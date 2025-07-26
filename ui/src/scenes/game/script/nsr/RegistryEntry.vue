<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
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
  <div class="flex size-full flex-col gap-4 overflow-x-hidden overflow-y-auto p-4 select-text">
    <div v-if="contents" class="nil-markdown" v-html="contents.readme"></div>
    <div v-if="code" class="nil-markdown" v-html="code"></div>
  </div>
</template>
