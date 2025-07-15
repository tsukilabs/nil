<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Button } from '@tb-dev/vue-components';
import type { MaybePromise } from '@tb-dev/utils';
import { usePlayerTurn } from '@/composables/player/usePlayerTurn';

defineProps<{
  isHost: boolean;
  onStartRound: () => MaybePromise<void>;
  onEndTurn: () => MaybePromise<void>;
}>();

const { t } = useI18n();

const { round } = NIL.round.refs();
const { player } = NIL.player.refs();

const isPlayerTurn = usePlayerTurn();
</script>

<template>
  <div class="flex items-center justify-center gap-4">
    <div
      v-if="player && round?.phase.kind === 'player'"
      class="flex flex-col items-center justify-center"
    >
      <span class="text-sm font-semibold">
        {{ `${t('round')} ${round.id}` }}
      </span>
      <span class="text-muted-foreground text-sm">
        {{ isPlayerTurn ? t('your-turn') : t('waiting-players') }}
      </span>
    </div>

    <Button v-if="isHost && round?.phase.kind === 'idle'" size="sm" @click="onStartRound">
      {{ t('start') }}
    </Button>
    <Button
      v-else-if="round?.phase.kind === 'player'"
      size="sm"
      :disabled="!isPlayerTurn"
      @click="onEndTurn"
    >
      {{ t('end-turn') }}
    </Button>
  </div>
</template>
