<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { exitGame } from '@/core/game';
import { Button } from '@tb-dev/vue-components';
import { useBreakpoints } from '@/composables/util/useBreakpoints';

const { sm } = useBreakpoints();

const { t } = useI18n();
</script>

<template>
  <div class="flex size-full flex-col items-center justify-center gap-2">
    <h1 class="font-nil -mt-16 mb-8 text-5xl font-extrabold sm:text-6xl md:text-7xl">
      <span>Call of Nil</span>
    </h1>
    <div class="flex flex-col gap-2">
      <Button variant="default" :size="sm ? 'default' : 'lg'">
        <RouterLink :to="{ name: 'host-game' as Scene }">
          {{ t('host-game') }}
        </RouterLink>
      </Button>
      <Button variant="secondary" :size="sm ? 'default' : 'lg'">
        <RouterLink :to="{ name: 'join-game' as Scene }">
          {{ t('join-game') }}
        </RouterLink>
      </Button>
      <Button variant="secondary" :size="sm ? 'default' : 'lg'">
        <RouterLink :to="{ name: 'settings' as Scene }">
          {{ t('settings') }}
        </RouterLink>
      </Button>
      <Button variant="secondary" :size="sm ? 'default' : 'lg'" @click="exitGame">
        <span>{{ t('exit') }}</span>
      </Button>
    </div>
  </div>
</template>
