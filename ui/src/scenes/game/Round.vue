<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { Button } from '@tb-dev/vue-components';
import type { MaybePromise } from '@tb-dev/utils';

defineProps<{ onTurnEnd: () => MaybePromise<void> }>();

const { player } = NIL.player.refs();
const { isPlayerTurn, round } = NIL.round.refs();
</script>

<template>
  <div class="flex items-center justify-center gap-4">
    <div v-if="round && player" class="flex flex-col items-center justify-center">
      <span class="text-sm font-semibold">{{ `${$t('round')} ${round.id}` }}</span>
      <span class="text-muted-foreground text-sm">
        {{ isPlayerTurn ? $t('your-turn') : $t('waiting-players') }}
      </span>
    </div>
    <Button size="sm" :disabled="!isPlayerTurn" @click="onTurnEnd">
      {{ $t('end-turn') }}
    </Button>
  </div>
</template>
