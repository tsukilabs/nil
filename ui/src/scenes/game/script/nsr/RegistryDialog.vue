<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useTemplateRef } from 'vue';
import { toPixel } from '@tb-dev/utils';
import { useHeight } from '@tb-dev/vue';
import SidebarButton from './SidebarButton.vue';
import {
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
  registry: readonly NsrScript[];
  loading: boolean;
  onEntryClick: (script: NsrScript) => MaybePromise<void>;
}>();

const open = defineModel<boolean>({ required: true });

const { t } = useI18n();

const content = useTemplateRef('contentEl');
const contentHeight = useHeight(content);

async function onClick(script: NsrScript) {
  if (!props.loading) {
    await props.onEntryClick(script);
    open.value = false;
  }
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogTrigger class="hidden"></DialogTrigger>
    <DialogContent>
      <VisuallyHidden>
        <DialogHeader>
          <DialogTitle>{{ t('registry', 2) }}</DialogTitle>
          <DialogDescription>
            {{ t('registry', 2) }}
          </DialogDescription>
        </DialogHeader>
      </VisuallyHidden>

      <div ref="contentEl" class="size-full px-0 py-4 overflow-hidden">
        <ScrollArea :style="{ height: toPixel(contentHeight - 20) }">
          <div
            v-for="entry of registry"
            :key="entry.id"
            class="w-full px-2"
          >
            <SidebarButton :entry @entry-click="onClick" />
          </div>
        </ScrollArea>
      </div>
    </DialogContent>
  </Dialog>
</template>
