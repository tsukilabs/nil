<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Action from './Action.vue';
import { useI18n } from 'vue-i18n';
import Sidebar from './Sidebar.vue';
import { asyncRef } from '@tb-dev/vue';
import { onBeforeMount, watch } from 'vue';
import { UseOnline } from '@vueuse/components';
import RegistryEntry from './RegistryEntry.vue';
import { createHighlighter } from '@/lib/editor';
import { useNsr } from '@/composables/script/useNsr';
import enUS from '@/locale/en-US/scenes/game/script/nsr.json';
import ptBR from '@/locale/pt-BR/scenes/game/script/nsr.json';
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
        <UseOnline #default="{ isOnline }: { isOnline: boolean }">
          <div v-if="isOnline" class="flex size-full items-center justify-between">
            <Sidebar :registry @entry-click="setCurrent" />

            <Separator orientation="vertical" />

            <div class="flex size-full flex-col pl-4">
              <Action
                :current
                :contents
                :loading
                class="py-4 pr-4"
                @execute="execute"
                @save="save"
                @dowload="download"
                @reload="reload"
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
        </UseOnline>
      </CardContent>
    </Card>
  </div>
</template>
