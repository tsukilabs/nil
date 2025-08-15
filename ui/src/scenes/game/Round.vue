<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import RoundState from './RoundState.vue';
import { Button } from '@tb-dev/vue-components';
import type { MaybePromise } from '@tb-dev/utils';
import { usePlayerTurn } from '@/composables/player/usePlayerTurn';
import { useBreakpoints } from '@/composables/util/useBreakpoints';

defineProps<{
  isHost: boolean;
  onStartRound: () => MaybePromise<void>;
  onFinishTurn: () => MaybePromise<void>;
}>();

const { t } = useI18n();

const { round } = NIL.round.refs();
const { player } = NIL.player.refs();

const isPlayerTurn = usePlayerTurn();

const { sm } = useBreakpoints();
</script>

<template>
  <div class="flex items-center justify-center gap-4">
    <RoundState v-if="sm && player && round?.state.kind === 'waiting'" />
    <Button
      v-if="isHost && round?.state.kind === 'idle'"
      size="sm"
      @click="onStartRound"
    >
      {{ t('start') }}
    </Button>
    <Button
      v-else-if="round?.state.kind === 'waiting'"
      size="sm"
      :disabled="!isPlayerTurn"
      @click="onFinishTurn"
    >
      {{ sm ? t('finish-turn') : t('finish') }}
    </Button>
  </div>
</template>
