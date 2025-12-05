<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useMutex } from '@tb-dev/vue';
import RoundState from './RoundState.vue';
import { Button } from '@tb-dev/vue-components';
import { useBreakpoints } from '@/composables/util/useBreakpoints';

const props = defineProps<{
  isHost: boolean;
  isPlayerTurn: boolean;
  isPlayerReady: boolean;
  onStartRound: () => Promise<void>;
  onTogglePlayerReady: () => Promise<void>;
}>();

const { t } = useI18n();

const { round } = NIL.round.refs();
const { player } = NIL.player.refs();

const { sm } = useBreakpoints();
const { locked, lock } = useMutex();

const start = () => lock(() => props.onStartRound());
const toggleReady = () => lock(() => props.onTogglePlayerReady());
</script>

<template>
  <div class="flex items-center justify-center gap-4">
    <RoundState v-if="sm && player && round?.state.kind === 'waiting'" :is-player-ready />

    <Button
      v-if="isHost && round?.state.kind === 'idle'"
      size="sm"
      :disabled="locked"
      @click="start"
    >
      <span>{{ t('start') }}</span>
    </Button>

    <Button
      v-else-if="round?.state.kind === 'waiting'"
      size="sm"
      :disabled="locked || !isPlayerTurn"
      @click="toggleReady"
    >
      <span v-if="isPlayerReady">{{ t('ready') }}</span>
      <span v-else>{{ sm ? t('finish-turn') : t('finish') }}</span>
    </Button>
  </div>
</template>
