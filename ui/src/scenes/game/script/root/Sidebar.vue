<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useHeight } from '@tb-dev/vue';
import { computed, useTemplateRef } from 'vue';
import { type MaybePromise, toPixel } from '@tb-dev/utils';
import { Button, ScrollArea } from '@tb-dev/vue-components';

defineProps<{
  scripts: readonly Script[];
  loading: boolean;
  onCreate: () => MaybePromise<void>;
  onScriptClick: (script: Script) => MaybePromise<void>;
}>();

const { t } = useI18n();

const container = useTemplateRef('containerEl');
const containerHeight = useHeight(container);

const footer = useTemplateRef('footerEl');
const footerHeight = useHeight(footer);

const contentHeight = computed(() => {
  return Math.max(containerHeight.value - footerHeight.value - 20, 0);
});

function toElementId(id: ScriptId) {
  return `player-script-${id}`;
}
</script>

<template>
  <div
    ref="containerEl"
    class="flex h-full w-50 min-w-50 flex-col justify-between overflow-hidden py-4"
  >
    <div class="size-full">
      <ScrollArea :style="{ height: toPixel(contentHeight) }">
        <div
          v-for="script of scripts"
          :id="toElementId(script.id)"
          :key="script.id"
          class="w-full px-2"
        >
          <Button
            variant="ghost"
            :disabled="loading"
            class="w-full justify-start font-normal"
            @click="() => onScriptClick(script)"
          >
            <span class="ellipsis">{{ script.name }}</span>
          </Button>
        </div>
      </ScrollArea>
    </div>

    <div ref="footerEl" class="h-20 p-4">
      <Button variant="outline" :disabled="loading" class="w-full" @click="onCreate">
        {{ t('new') }}
      </Button>
    </div>
  </div>
</template>
