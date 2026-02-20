<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { onUnmounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { useBreakpoints } from '@/composables/useBreakpoints';
import { useTerminal } from '@/composables/scripts/useTerminal';
import { createHighlighter, disposeHighlighter, toHtml } from '@/lib/highlighter';
import { Button, Card, CardContent, CardHeader, CardTitle, Textarea } from '@tb-dev/vue-components';

const { t } = useI18n();

const { player } = NIL.player.refs();
const { md } = useBreakpoints();

const { chunk, lines, loading, execute, clear } = useTerminal();

await createHighlighter();

async function onEnter(e: Event) {
  if (
    e instanceof KeyboardEvent &&
    e.key.toLowerCase() === 'enter' &&
    !e.shiftKey
  ) {
    await execute();
  }
}

onUnmounted(() => disposeHighlighter());
</script>

<template>
  <div class="game-layout">
    <Card class="size-full">
      <CardHeader>
        <CardTitle>
          <span>{{ t('terminal') }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="size-full flex flex-col gap-4 md:gap-6 overflow-hidden px-2 py-0">
        <div class="font-ubuntu-mono size-full overflow-x-hidden overflow-y-auto">
          <div v-for="line of lines" :key="line.id">
            <div v-if="line.kind === 'input'" class="terminal-input flex gap-2">
              <span class="font-semibold">{{ player?.id }}: </span>
              <span v-html="toHtml(line.content)"></span>
            </div>

            <span v-else class="terminal-output">{{ line.content }} </span>
          </div>
        </div>

        <div class="h-[50px] max-w-full flex items-center justify-between gap-2 px-1 sm:px-2 pb-2">
          <Textarea
            v-model="chunk"
            type="text"
            :disabled="loading"
            spellcheck="false"
            autocapitalize="off"
            autocomplete="off"
            rows="1"
            class="h-full! min-h-0! resize-none"
            @keydown.enter="onEnter"
          />
          <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
            <Button :disabled="!chunk || loading" @click="execute">
              {{ t('execute') }}
            </Button>
            <Button v-if="md" variant="destructive" :disabled="loading" @click="clear">
              {{ t('clear') }}
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<style scoped>
:deep(.shiki), .terminal-output {
  background-color: transparent !important;
  white-space: normal;
  text-wrap: pretty;
  overflow-wrap: anywhere;

  -webkit-user-select: text;
  user-select: text;
}

div:has(.terminal-input) + div:has(.terminal-output),
div:has(.terminal-output) + div:has(.terminal-input) {
  padding-top: 0.5rem;
}
</style>
