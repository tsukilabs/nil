<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Action from './Action.vue';
import Editor from './Editor.vue';
import Sidebar from './Sidebar.vue';
import type { CodeEditor } from '@/lib/editor';
import { onBeforeMount, shallowRef, watch } from 'vue';
import { useScripts } from '@/composables/script/useScripts';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
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

const { lg } = useBreakpoints();

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
          <template v-if="lg">
            <Sidebar :scripts :loading @create="createScript" @script-click="onScriptClick" />
            <Separator orientation="vertical" />
          </template>

          <div class="flex size-full flex-col pl-4">
            <div class="flex items-center justify-between gap-4 lg:gap-12 py-4 pr-4">
              <div class="flex w-full items-center lg:max-w-96">
                <Input v-if="current" v-model="current.name" type="text" />
              </div>
              <Action
                :scripts
                :current
                :loading
                @create="createScript"
                @execute="executeScript"
                @import="importScripts"
                @export="exportScripts"
                @remove="removeScript"
                @save="saveScript"
                @script-click="onScriptClick"
              />
            </div>

            <Editor v-show="current" v-model="editor" />
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
