<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';

defineProps<{
  isPlayerReady: boolean;
}>();

const { t } = useI18n();

const { round } = NIL.round.refs();
</script>

<template>
  <div class="flex flex-col items-center justify-center">
    <span v-if="round" class="text-sm font-semibold">
      {{ `${t('round')} ${round.id}` }}
    </span>
    <span class="text-muted-foreground text-sm">
      {{ isPlayerReady ? t('your-turn') : t('waiting-players') }}
    </span>
  </div>
</template>
