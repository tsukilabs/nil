<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Action from './Action.vue';
import Editor from './Editor.vue';
import Sidebar from './Sidebar.vue';
import type { Option } from '@tb-dev/utils';
import type { CodeEditor } from '@/lib/editor';
import { onBeforeMount, shallowRef, watch } from 'vue';
import { useScripts } from '@/composables/script/useScripts';
import { Card, CardContent, Input, Separator } from '@tb-dev/vue-components';

const editor = shallowRef<Option<CodeEditor>>();

const {
  scripts,
  current,
  loading,
  loadScripts,
  executeScript,
  saveScript,
  createScript,
  removeScript,
  importScripts,
  exportScripts,
  setCurrent,
} = useScripts(editor);

watch(scripts, (it) => {
  if (!current.value && it.length > 0) {
    const script = it.at(0);
    if (script) setCurrent(script);
  }
});

watch(current, (script) => {
  editor.value?.setValue(script?.code ?? '');
});

onBeforeMount(loadScripts);

function onScriptClick(script: Script) {
  if (!loading.value) {
    setCurrent(script);
  }
}
</script>

<template>
  <div class="game-layout">
    <Card class="size-full p-0">
      <CardContent class="size-full p-0">
        <div class="flex size-full items-center justify-between">
          <Sidebar :scripts :loading @create="createScript" @script-click="onScriptClick" />

          <Separator orientation="vertical" />

          <div class="flex size-full flex-col pl-4">
            <div class="flex items-center justify-between gap-12 py-4 pr-4">
              <div class="flex w-full items-center lg:max-w-96">
                <Input v-if="current" v-model="current.name" type="text" />
              </div>
              <Action
                :current
                :loading
                class="hidden lg:grid"
                @execute="executeScript"
                @import="importScripts"
                @export="exportScripts"
                @remove="removeScript"
                @save="saveScript"
              />
            </div>

            <Editor v-show="current" v-model="editor" />
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
