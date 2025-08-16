<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Action from './Action.vue';
import { useI18n } from 'vue-i18n';
import Sidebar from './Sidebar.vue';
import { asyncRef } from '@tb-dev/vue';
import { useOnline } from '@vueuse/core';
import { onBeforeMount, watch } from 'vue';
import RegistryEntry from './RegistryEntry.vue';
import { createHighlighter } from '@/lib/editor';
import { useNsr } from '@/composables/script/useNsr';
import enUS from '@/locale/en-US/scenes/game/script/nsr.json';
import ptBR from '@/locale/pt-BR/scenes/game/script/nsr.json';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import { Card, CardContent, Separator } from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const {
  registry,
  current,
  contents,
  loading,
  loadRegistry,
  execute,
  save,
  download,
  reload,
  setCurrent,
} = useNsr();

const { state: highlighter } = asyncRef(null, createHighlighter);

const isOnline = useOnline();
const { lg } = useBreakpoints();

watch(registry, () => {
  if (registry.value.length > 0 && !current.value) {
    setCurrent(registry.value[0]);
  }
});

onBeforeMount(loadRegistry);
</script>

<template>
  <div class="game-layout">
    <Card class="size-full p-0">
      <CardContent class="relative size-full overflow-hidden rounded-xl p-0">
        <div v-if="isOnline" class="flex size-full items-center justify-between">
          <template v-if="lg">
            <Sidebar :registry @entry-click="setCurrent" />
            <Separator orientation="vertical" />
          </template>

          <div class="flex size-full flex-col pl-4">
            <Action
              :registry
              :current
              :contents
              :loading
              class="py-4 pr-4"
              @execute="execute"
              @save="save"
              @download="download"
              @reload="reload"
              @entry-click="setCurrent"
            />

            <RegistryEntry v-if="highlighter" :contents :highlighter />
          </div>
        </div>

        <div v-else class="absolute inset-0 flex flex-col items-center justify-center gap-2">
          <div class="text-4xl">{{ t('offline') }}</div>
          <div class="text-muted-foreground text-xl">
            {{ t('need-connection') }}
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
