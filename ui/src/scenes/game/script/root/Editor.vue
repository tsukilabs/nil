<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { until, useElementBounding } from '@vueuse/core';
import { onMounted, onUnmounted, useTemplateRef, watch } from 'vue';
import { type CodeEditor, createEditor, disposeEditor } from '@/lib/editor';

const editor = defineModel<Option<CodeEditor>>({ required: true });

const container = useTemplateRef('editorEl');
const { width, height } = useElementBounding(container, {
  immediate: true,
  reset: true,
  updateTiming: 'next-frame',
  windowResize: true,
  windowScroll: true,
});

watch([width, height], ([x, y]) => {
  requestAnimationFrame(() => setLayout(x, y));
});

onMounted(async () => {
  await until(container).toBeTruthy();
  editor.value ??= await createEditor(container.value!);
  requestAnimationFrame(() => setLayout());
});

onUnmounted(() => {
  disposeEditor();
  editor.value = null;
});

function setLayout(x?: number, y?: number) {
  x ??= width.value;
  y ??= height.value;
  editor.value?.layout({ width: x - 20, height: y - 20 });
}
</script>

<template>
  <div ref="editorEl" class="size-full"></div>
</template>
