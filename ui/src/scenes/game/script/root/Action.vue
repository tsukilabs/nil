<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import ActionTooltip from './ActionTooltip.vue';
import { Button } from '@tb-dev/vue-components';
import { FileDownIcon, FileUpIcon, GlobeIcon, PlayIcon, SaveIcon, Trash2Icon } from 'lucide-vue-next';

defineProps<{
  current: Option<Script>;
  loading: boolean;
  onExecute: () => MaybePromise<void>;
  onImport: () => MaybePromise<void>;
  onExport: () => MaybePromise<void>;
  onRemove: () => MaybePromise<void>;
  onSave: () => MaybePromise<void>;
}>();

const { t } = useI18n();
</script>

<template>
  <div class="grid-cols-6 gap-2">
    <ActionTooltip :label="t('execute')">
      <Button variant="ghost" size="icon" :disabled="!current?.id || loading" @click="onExecute">
        <PlayIcon />
      </Button>
    </ActionTooltip>

    <ActionTooltip :label="t('save')">
      <Button variant="ghost" size="icon" :disabled="!current || loading" @click="onSave">
        <SaveIcon />
      </Button>
    </ActionTooltip>

    <ActionTooltip :label="t('remove')">
      <Button variant="ghost" size="icon" :disabled="!current || loading" @click="onRemove">
        <Trash2Icon />
      </Button>
    </ActionTooltip>

    <ActionTooltip :label="t('import')">
      <Button variant="ghost" size="icon" :disabled="loading" @click="onImport">
        <FileDownIcon />
      </Button>
    </ActionTooltip>

    <ActionTooltip :label="t('export')">
      <Button variant="ghost" size="icon" :disabled="!current || loading" @click="onExport">
        <FileUpIcon />
      </Button>
    </ActionTooltip>

    <ActionTooltip label="NSR">
      <Button variant="ghost" size="icon" :disabled="loading">
        <RouterLink :to="{ name: 'nsr' satisfies ScriptScene }">
          <GlobeIcon />
        </RouterLink>
      </Button>
    </ActionTooltip>
  </div>
</template>
