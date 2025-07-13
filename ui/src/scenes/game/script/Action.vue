<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Button } from '@tb-dev/vue-components';
import type { MaybePromise, Option } from '@tb-dev/utils';

defineProps<{
  current: Option<Script>;
  loading: boolean;
  onImport: () => MaybePromise<void>;
  onExport: () => MaybePromise<void>;
  onRemove: () => MaybePromise<void>;
  onSave: () => MaybePromise<void>;
}>();

const { t } = useI18n();
</script>

<template>
  <div class="grid-cols-4 gap-2">
    <Button size="sm" :disabled="!current || loading" @click="onSave">
      {{ t('save') }}
    </Button>
    <Button size="sm" :disabled="loading" @click="onImport">
      {{ t('import') }}
    </Button>
    <Button size="sm" :disabled="!current || loading" @click="onExport">
      {{ t('export') }}
    </Button>
    <Button size="sm" variant="destructive" :disabled="!current || loading" @click="onRemove">
      {{ t('remove') }}
    </Button>
  </div>
</template>
