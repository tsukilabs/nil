<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import { useI18n } from 'vue-i18n';
import { exitGame } from '@/core/game';
import enUS from '@/locale/en-US/scenes/home.json';
import ptBR from '@/locale/pt-BR/scenes/home.json';
import { useUpdate } from '@/composables/util/useUpdate';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import { Alert, AlertDescription, AlertTitle, Button } from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const update = useUpdate();
const { sm, md } = useBreakpoints();
</script>

<template>
  <div class="flex size-full flex-col items-center justify-center gap-2">
    <h1 class="font-nil -mt-16 mb-8 text-5xl font-extrabold sm:text-6xl md:text-7xl">
      <span>Call of Nil</span>
    </h1>

    <div class="flex flex-col gap-2">
      <Button
        variant="default"
        :size="sm ? 'default' : 'lg'"
        role="link"
        tabindex="0"
        @click="() => go('host-local-game')"
      >
        <span>{{ t('host-game') }}</span>
      </Button>

      <Button
        variant="secondary"
        :size="sm ? 'default' : 'lg'"
        role="link"
        tabindex="0"
        @click="() => go('join-local-game')"
      >
        <span>{{ t('join-game') }}</span>
      </Button>

      <Button
        variant="secondary"
        :size="sm ? 'default' : 'lg'"
        role="link"
        tabindex="0"
        @click="() => go('settings')"
      >
        <span>{{ t('settings') }}</span>
      </Button>

      <Button variant="secondary" :size="sm ? 'default' : 'lg'" @click="exitGame">
        <span>{{ t('exit') }}</span>
      </Button>
    </div>

    <Alert v-if="update && md" class="w-max fixed bottom-safe-4 right-safe-4 py-4">
      <AlertTitle>{{ t('update-available') }}</AlertTitle>
      <AlertDescription class="gap-2">
        <span>{{ t('version-ready', [update.version]) }}</span>
        <div class="grid grid-cols-2 items-center gap-2 justify-self-end">
          <Button variant="default" size="sm" @click="() => update?.install()">
            <span>{{ t('update') }}</span>
          </Button>

          <Button variant="secondary" size="sm" @click="() => update?.openChangelog()">
            <span>{{ t('whats-new') }}</span>
          </Button>
        </div>
      </AlertDescription>
    </Alert>
  </div>
</template>
