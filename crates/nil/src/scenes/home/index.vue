<script setup lang="ts">
import { Game } from '@/core/game';
import { useLocale } from '@/locale';
import { Button, ButtonLink } from '@/components';

const { t } = useLocale();
const game = Game.use();
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
      <Button variant="secondary" @click="() => game.exit()">
        <span>{{ t('exit') }}</span>
      </Button>
    </div>
  </div>
</template>
