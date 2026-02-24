<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Chunk from './Chunk.vue';
import Sheet from './Sheet.vue';
import { useI18n } from 'vue-i18n';
import Sidebar from './Sidebar.vue';
import { useToggle } from '@vueuse/core';
import HeaderMenu from './HeaderMenu.vue';
import { onUnmounted, shallowRef, watch } from 'vue';
import { ScriptImpl } from '@/core/model/scripts/script';
import { useScripts } from '@/composables/scripts/useScripts';
import { handleError, useBreakpoints, useMutex } from '@tb-dev/vue';
import { createHighlighter, disposeHighlighter } from '@/lib/highlighter';
import { Card, CardContent, CardHeader, CardTitle, Separator } from '@tb-dev/vue-components';

const { t } = useI18n();

const {
  scripts,
  load: loadScripts,
  loading: isLoadingScripts,
} = useScripts();

const currentScript = shallowRef<Option<ScriptImpl>>();
const { locked, ...mutex } = useMutex();

const { md } = useBreakpoints();

const [isSheetOpen, toggleSheet] = useToggle(false);

watch(scripts, (it) => void (currentScript.value ??= it.at(0)));

onUnmounted(() => disposeHighlighter());

await createHighlighter();

async function execute() {
  try {
    await mutex.acquire();
    await currentScript.value?.execute();
  }
  catch (err) {
    handleError(err);
  }
  finally {
    mutex.release();
  }
}

async function remove() {
  try {
    await mutex.acquire();
    await currentScript.value?.remove();
    currentScript.value = null;
    await loadScripts();
    currentScript.value = scripts.value.at(0);
  }
  catch (err) {
    handleError(err);
  }
  finally {
    mutex.release();
  }
}

function onScriptClick(script: ScriptImpl) {
  currentScript.value = script;
}
</script>

<template>
  <div class="game-layout">
    <Card class="size-full">
      <CardHeader>
        <CardTitle>
          <div class="flex items-center justify-between">
            <span class="w-full">{{ t('script', 2) }}</span>
            <HeaderMenu
              :current-script
              :disabled="locked || isLoadingScripts"
              :toggle-sheet
              @execute="execute"
              @remove="remove"
            />
          </div>
        </CardTitle>
      </CardHeader>

      <CardContent class="size-full flex overflow-auto px-2 py-0">
        <template v-if="md">
          <Sidebar
            :scripts
            :disabled="locked || isLoadingScripts"
            @script-click="onScriptClick"
          />
          <Separator orientation="vertical" />
        </template>
        <Chunk v-if="currentScript" :script="currentScript" />
      </CardContent>
    </Card>

    <Sheet
      v-if="!md"
      v-model:open="isSheetOpen"
      :scripts
      :disabled="locked || isLoadingScripts"
      @script-click="onScriptClick"
    />
  </div>
</template>
