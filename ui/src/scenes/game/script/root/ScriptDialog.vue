<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useTemplateRef } from 'vue';
import { toPixel } from '@tb-dev/utils';
import { useHeight } from '@tb-dev/vue';
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  ScrollArea,
  VisuallyHidden,
} from '@tb-dev/vue-components';

const props = defineProps<{
  scripts: readonly Script[];
  loading: boolean;
  onScriptClick: (script: Script) => MaybePromise<void>;
}>();

const open = defineModel<boolean>({ required: true });

const { t } = useI18n();

const content = useTemplateRef('contentEl');
const contentHeight = useHeight(content);

async function onClick(script: Script) {
  await props.onScriptClick(script);
  open.value = false;
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogTrigger class="hidden"></DialogTrigger>
    <DialogContent>
      <VisuallyHidden>
        <DialogHeader>
          <DialogTitle>{{ t('script', 2) }}</DialogTitle>
          <DialogDescription>
            {{ t('script', 2) }}
          </DialogDescription>
        </DialogHeader>
      </VisuallyHidden>

      <div ref="contentEl" class="size-full px-0 py-4 overflow-hidden">
        <ScrollArea :style="{ height: toPixel(contentHeight - 20) }">
          <div
            v-for="script of scripts"
            :key="script.id"
            class="w-full px-2"
          >
            <Button
              variant="ghost"
              :disabled="loading"
              class="w-full justify-start font-normal"
              @click="() => onClick(script)"
            >
              <span class="ellipsis">{{ script.name }}</span>
            </Button>
          </div>
        </ScrollArea>
      </div>
    </DialogContent>
  </Dialog>
</template>
