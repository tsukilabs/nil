<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useToggle } from '@vueuse/core';
import { shareText } from 'tauri-plugin-mobile';
import RegistryDialog from './RegistryDialog.vue';
import ActionTooltip from '../root/ActionTooltip.vue';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import type { ScriptContents } from '@/composables/script/useNsr';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import { CopyIcon, DownloadIcon, MenuIcon, PlayIcon, RefreshCwIcon, SaveIcon, Share2Icon } from 'lucide-vue-next';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@tb-dev/vue-components';

const props = defineProps<{
  registry: readonly NsrScript[];
  current: Option<NsrScript>;
  contents: Option<ScriptContents>;
  loading: boolean;
  onDownload: () => MaybePromise<void>;
  onExecute: () => MaybePromise<void>;
  onReload: () => MaybePromise<void>;
  onSave: () => MaybePromise<void>;
  onEntryClick: (script: NsrScript) => MaybePromise<void>;
}>();

const { t } = useI18n();

const { lg } = useBreakpoints();
const [isRegistryDialogOpen, toggleRegistryDialog] = useToggle(false);
const isReadonly = computed(() => props.current?.frontmatter.readonly ?? true);

const desktop = window.__DESKTOP__;
const mobile = window.__MOBILE__;

function onCopy() {
  if (props.contents?.script) {
    writeText(props.contents.script).err();
  }
}

async function share() {
  if (mobile && props.current && props.contents?.script) {
    await shareText({
      title: props.current.frontmatter.name,
      text: props.contents.script,
      mimeType: 'text/plain',
    });
  }
}
</script>

<template>
  <div class="flex items-center justify-end">
    <div v-if="lg" class="grid max-w-fit grid-cols-5 gap-2">
      <ActionTooltip :label="t('execute')">
        <Button variant="ghost" size="icon" :disabled="isReadonly || loading" @click="onExecute">
          <PlayIcon stroke-width="1.5px" />
        </Button>
      </ActionTooltip>

      <ActionTooltip :label="t('save')">
        <Button variant="ghost" size="icon" :disabled="loading" @click="onSave">
          <SaveIcon stroke-width="1.5px" />
        </Button>
      </ActionTooltip>

      <ActionTooltip :label="t('copy')">
        <Button variant="ghost" size="icon" :disabled="!contents?.script" @click="onCopy">
          <CopyIcon stroke-width="1.5px" />
        </Button>
      </ActionTooltip>

      <ActionTooltip v-if="desktop" :label="t('download')">
        <Button variant="ghost" size="icon" :disabled="loading" @click="onDownload">
          <DownloadIcon stroke-width="1.5px" />
        </Button>
      </ActionTooltip>

      <ActionTooltip v-if="mobile" :label="t('share')">
        <Button
          variant="ghost"
          size="icon"
          :disabled="loading || !props.current || !props.contents?.script"
          @click="share"
        >
          <Share2Icon stroke-width="1.5px" />
        </Button>
      </ActionTooltip>

      <ActionTooltip :label="t('reload')">
        <Button variant="ghost" size="icon" :disabled="loading" @click="onReload">
          <RefreshCwIcon stroke-width="1.5px" />
        </Button>
      </ActionTooltip>
    </div>

    <template v-else>
      <RegistryDialog
        v-model="isRegistryDialogOpen"
        :registry
        :loading
        @entry-click="onEntryClick"
      />

      <DropdownMenu>
        <DropdownMenuTrigger as-child>
          <Button variant="ghost" size="icon">
            <MenuIcon stroke-width="1.5px" />
          </Button>
        </DropdownMenuTrigger>

        <DropdownMenuContent
          align="end"
          :align-offset="-15"
          side="bottom"
          :side-offset="5"
          class="w-56"
        >
          <DropdownMenuGroup>
            <DropdownMenuItem
              :disabled="loading || isRegistryDialogOpen || registry.length === 0"
              @click="toggleRegistryDialog"
            >
              {{ t('select') }}
            </DropdownMenuItem>

            <DropdownMenuItem :disabled="isReadonly || loading" @click="onExecute">
              {{ t('execute') }}
            </DropdownMenuItem>

            <DropdownMenuItem :disabled="loading" @click="onSave">
              {{ t('save') }}
            </DropdownMenuItem>

            <DropdownMenuItem :disabled="!contents?.script" @click="onCopy">
              {{ t('copy') }}
            </DropdownMenuItem>

            <DropdownMenuItem v-if="desktop" :disabled="loading" @click="onDownload">
              {{ t('download') }}
            </DropdownMenuItem>

            <DropdownMenuItem
              v-if="mobile"
              :disabled="loading || !props.current || !props.contents?.script"
              @click="share"
            >
              {{ t('share') }}
            </DropdownMenuItem>
          </DropdownMenuGroup>
        </DropdownMenuContent>
      </DropdownMenu>
    </template>
  </div>
</template>
