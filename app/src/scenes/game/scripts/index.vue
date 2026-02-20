<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import Sidebar from './Sidebar.vue';
import { Card, CardContent, CardHeader, CardTitle } from '@tb-dev/vue-components';

const { t } = useI18n();
</script>

<template>
  <div class="game-layout">
    <Card class="size-full">
      <CardHeader>
        <CardTitle>
          <span>{{ t('script', 2) }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="overflow-hidden"> </CardContent>
    </Card>
  </div>
</template>
