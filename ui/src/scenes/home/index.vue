<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { exitGame } from '@/core/game';
import { Button, ButtonLink } from '@tb-dev/vue-components';

const { t } = useI18n();
</script>

<template>
  <div class="flex size-full flex-col items-center justify-center gap-2">
    <h1 class="font-nil -mt-16 mb-8 text-4xl font-extrabold sm:text-5xl md:text-6xl lg:text-8xl">
      {{ t('game-title') }}
    </h1>
    <div class="flex flex-col gap-2">
      <ButtonLink to="join-game" variant="default">
        <span>{{ t('join-game') }}</span>
      </ButtonLink>
      <ButtonLink to="host-game" variant="secondary">
        <span>{{ t('host-game') }}</span>
      </ButtonLink>
      <ButtonLink to="settings" variant="secondary">
        <span>{{ t('settings') }}</span>
      </ButtonLink>
      <Button variant="secondary" @click="exitGame">
        <span>{{ t('exit') }}</span>
      </Button>
    </div>
  </div>
</template>
