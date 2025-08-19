<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Button, cn } from '@tb-dev/vue-components';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import enUS from '@/locale/en-US/scenes/game/infrastructure/prefecture.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/prefecture.json';
import type { BuildingImpl } from '@/core/model/infrastructure/building/abstract';

const props = defineProps<{
  building: BuildingImpl;
  canBuild: boolean;
  canDemolish: boolean;
  isPlayerTurn: boolean;
  loading: boolean;
  class?: string;
  onOrder: (kind: PrefectureBuildOrderKind) => MaybePromise<void>;
  onToggle: () => void;
}>();

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const { sm } = useBreakpoints();
</script>

<template>
  <div :class="cn('grid max-w-fit grid-cols-3 items-center justify-start gap-4', props.class)">
    <Button
      variant="default"
      :size="sm ? 'sm' : 'xs'"
      :disabled="!canBuild"
      class="max-w-32"
      @click="() => onOrder('construction')"
    >
      <span>{{ t('build') }}</span>
    </Button>
    <Button
      variant="secondary"
      :size="sm ? 'sm' : 'xs'"
      :disabled="loading || !isPlayerTurn"
      class="max-w-32"
      @click="() => onToggle()"
    >
      <span>{{ building.enabled ? t('disable') : t('enable') }}</span>
    </Button>
    <Button
      variant="destructive"
      :size="sm ? 'sm' : 'xs'"
      :disabled="!canDemolish"
      class="max-w-32"
      @click="() => onOrder('demolition')"
    >
      <span>{{ t('demolish') }}</span>
    </Button>
  </div>
</template>
