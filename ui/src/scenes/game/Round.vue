<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useMutex } from '@tb-dev/vue';
import RoundState from './RoundState.vue';
import { Button } from '@tb-dev/vue-components';
import { usePlayerTurn } from '@/composables/player/usePlayerTurn';
import { useBreakpoints } from '@/composables/util/useBreakpoints';

const props = defineProps<{
  isHost: boolean;
  onStartRound: () => Promise<void>;
  onFinishTurn: () => Promise<void>;
}>();

const { t } = useI18n();

const { round } = NIL.round.refs();
const { player } = NIL.player.refs();

const isPlayerTurn = usePlayerTurn();
const { locked, lock } = useMutex();

const { sm } = useBreakpoints();

const start = () => lock(() => props.onStartRound());
const finish = () => lock(() => props.onFinishTurn());
</script>

<template>
  <div class="flex items-center justify-center gap-4">
    <RoundState v-if="sm && player && round?.state.kind === 'waiting'" />
    <Button
      v-if="isHost && round?.state.kind === 'idle'"
      size="sm"
      :disabled="locked"
      @click="start"
    >
      {{ t('start') }}
    </Button>
    <Button
      v-else-if="round?.state.kind === 'waiting'"
      size="sm"
      :disabled="locked || !isPlayerTurn"
      @click="finish"
    >
      {{ sm ? t('finish-turn') : t('finish') }}
    </Button>
  </div>
</template>
