<script setup lang="ts">
import { useLocale } from '@/locale';
import { Button } from '@/components';
import { exit } from '@tauri-apps/plugin-process';

const { t } = useLocale();

function onExit() {
  exit(0).handleError();
}
</script>

<template>
  <div class="flex size-full flex-col items-center justify-center gap-2">
    <h1 class="font-nil -mt-16 mb-8 text-4xl font-extrabold sm:text-5xl md:text-6xl lg:text-8xl">
      {{ t('game-title') }}
    </h1>
    <div class="flex flex-col gap-2">
      <Button @click="() => $go('join-game')">{{ t('join-game') }}</Button>
      <Button variant="secondary" @click="() => $go('host-game')">{{ t('host-game') }}</Button>
      <Button variant="secondary" disabled @click="() => $go('settings')">
        {{ t('settings') }}
      </Button>
      <Button variant="secondary" @click="onExit">{{ t('exit') }}</Button>
    </div>
  </div>
</template>
