<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { ScriptImpl } from '@/core/model/scripts/script';
import {
  Button,
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
  VisuallyHidden,
} from '@tb-dev/vue-components';

const props = defineProps<{
  scripts: readonly ScriptImpl[];
  disabled: boolean;
  onScriptClick: (script: ScriptImpl) => MaybePromise<void>;
}>();

const isOpen = defineModel<boolean>('open', { required: true });

const { t } = useI18n();

async function set(script: ScriptImpl) {
  isOpen.value = false;
  await props.onScriptClick(script);
}
</script>

<template>
  <Sheet v-model:open="isOpen">
    <SheetTrigger class="hidden"></SheetTrigger>
    <SheetContent disable-outside-pointer-events class="pb-safe-4">
      <SheetHeader class="pb-0">
        <SheetTitle>{{ t('script', 2) }}</SheetTitle>
        <VisuallyHidden>
          <SheetDescription>
            {{ t('script', 2) }}
          </SheetDescription>
        </VisuallyHidden>
      </SheetHeader>

      <SheetClose class="hidden" />

      <div class="h-full overflow-x-hidden overflow-y-auto pt-2">
        <div v-for="script of scripts" :key="script.path" class="w-full px-2">
          <Button
            variant="ghost"
            :disabled
            class="w-full justify-start font-normal"
            @click="() => set(script)"
          >
            <span class="ellipsis">{{ script.name }}</span>
          </Button>
        </div>
      </div>
    </SheetContent>
  </Sheet>
</template>
