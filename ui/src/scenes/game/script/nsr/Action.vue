<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Button } from '@tb-dev/vue-components';
import type { MaybePromise } from '@tb-dev/utils';
import ActionTooltip from '../root/ActionTooltip.vue';
import { DownloadIcon, FileCodeIcon, PlayIcon, RefreshCwIcon, SaveIcon } from 'lucide-vue-next';

defineProps<{
  loading: boolean;
  onDowload: () => MaybePromise<void>;
  onExecute: () => MaybePromise<void>;
  onReload: () => MaybePromise<void>;
  onSave: () => MaybePromise<void>;
}>();

const { t } = useI18n();
</script>

<template>
  <div class="flex items-center justify-end">
    <div class="grid max-w-fit grid-cols-5 gap-2">
      <ActionTooltip :label="t('execute')">
        <Button variant="ghost" size="icon" :disabled="loading" @click="onExecute">
          <PlayIcon />
        </Button>
      </ActionTooltip>

      <ActionTooltip :label="t('save')">
        <Button variant="ghost" size="icon" :disabled="loading" @click="onSave">
          <SaveIcon />
        </Button>
      </ActionTooltip>

      <ActionTooltip :label="t('download')">
        <Button variant="ghost" size="icon" :disabled="loading" @click="onDowload">
          <DownloadIcon />
        </Button>
      </ActionTooltip>

      <ActionTooltip :label="t('reload')">
        <Button variant="ghost" size="icon" :disabled="loading" @click="onReload">
          <RefreshCwIcon />
        </Button>
      </ActionTooltip>

      <ActionTooltip :label="t('script', 2)">
        <Button variant="ghost" size="icon" :disabled="loading">
          <RouterLink :to="{ name: 'script' satisfies ScriptScene }">
            <FileCodeIcon />
          </RouterLink>
        </Button>
      </ActionTooltip>
    </div>
  </div>
</template>
