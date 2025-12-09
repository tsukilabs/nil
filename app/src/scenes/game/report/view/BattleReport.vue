<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import type { BattleReportImpl } from '@/core/model/report/battle-report';

defineProps<{
  report: BattleReportImpl;
}>();
</script>

<template>
  <div>{{ JSON.stringify(report, null, 4) }}</div>
</template>
