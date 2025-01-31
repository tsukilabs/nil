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
    <h1 class="-mt-16 mb-8 text-4xl font-extrabold md:text-6xl">{{ t('title') }}</h1>
    <div class="flex flex-col gap-2">
      <Button>{{ t('home.join') }}</Button>
      <Button variant="secondary" disabled>{{ t('home.host') }}</Button>
      <Button variant="secondary" @click="onExit">{{ t('home.exit') }}</Button>
    </div>
  </div>
</template>
