<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { Button } from '@tb-dev/vue-components';
import type { ScriptImpl } from '@/core/model/scripts/script';

defineProps<{
  scripts: readonly ScriptImpl[];
  disabled: boolean;
  onScriptClick: (script: ScriptImpl) => MaybePromise<void>;
}>();
</script>

<template>
  <div class="h-full min-w-50 max-w-60 overflow-hidden">
    <div class="size-full overflow-x-hidden overflow-y-auto">
      <div v-for="script of scripts" :key="script.path" class="w-full px-2">
        <Button
          variant="ghost"
          :disabled
          class="w-full justify-start font-normal"
          @click="() => onScriptClick(script)"
        >
          <span class="ellipsis">{{ script.name }}</span>
        </Button>
      </div>
    </div>
  </div>
</template>
