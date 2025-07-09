<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import * as commands from '@/commands';
import { useHeight } from '@tb-dev/vue';
import { handleError } from '@/lib/error';
import { computed, useTemplateRef } from 'vue';
import { type MaybePromise, toPixel } from '@tb-dev/utils';
import { Button, ScrollArea } from '@tb-dev/vue-components';

const props = defineProps<{
  scripts: Script[];
  onCreate: (id: ScriptId) => MaybePromise<unknown>;
  onScriptClick: (script: Script) => MaybePromise<unknown>;
  waitToLoad: () => Promise<void>;
}>();

const loading = defineModel<boolean>('loading', { required: true });

const { t } = useI18n();

const { player } = NIL.player.refs();

const container = useTemplateRef('containerEl');
const containerHeight = useHeight(container);

const footer = useTemplateRef('footerEl');
const footerHeight = useHeight(footer);

const contentHeight = computed(() => {
  const height = containerHeight.value - footerHeight.value;
  return Math.max(height - 20, 0);
});

async function createScript() {
  await props.waitToLoad();
  if (player.value) {
    try {
      loading.value = true;
      const id = await commands.addScript({
        id: 0,
        name: 'Script',
        code: '',
        owner: player.value.id,
      });

      await props.onCreate(id);
      const elementId = `#${toElementId(id)}`;
      void container.value?.waitScroll(elementId, {
        timeout: 1000,
        throwOnTimeout: false,
      });
    } catch (err) {
      handleError(err);
    } finally {
      loading.value = false;
    }
  }
}

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
      <Button variant="outline" :disabled="!player || loading" class="w-full" @click="createScript">
        {{ t('new') }}
      </Button>
    </div>
  </div>
</template>
