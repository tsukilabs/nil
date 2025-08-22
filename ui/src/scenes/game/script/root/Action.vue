<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import { useI18n } from 'vue-i18n';
import { useToggle } from '@vueuse/core';
import ScriptDialog from './ScriptDialog.vue';
import { shareText } from 'tauri-plugin-mobile';
import ActionTooltip from './ActionTooltip.vue';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import {
  FileDownIcon,
  FileUpIcon,
  GlobeIcon,
  MenuIcon,
  PlayIcon,
  SaveIcon,
  Share2Icon,
  Trash2Icon,
} from 'lucide-vue-next';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@tb-dev/vue-components';

const props = defineProps<{
  scripts: readonly Script[];
  current: Option<Script>;
  loading: boolean;
  onCreate: () => MaybePromise<void>;
  onExecute: () => MaybePromise<void>;
  onImport: () => MaybePromise<void>;
  onExport: () => MaybePromise<void>;
  onRemove: () => MaybePromise<void>;
  onSave: () => MaybePromise<void>;
  onScriptClick: (script: Script) => MaybePromise<void>;
}>();

const { t } = useI18n();

const { lg } = useBreakpoints();
const [isScriptDialogOpen, toggleScriptDialog] = useToggle(false);

const desktop = window.__DESKTOP__;
const mobile = window.__MOBILE__;

async function share() {
  if (mobile && props.current?.code) {
    await shareText({
      title: props.current.name,
      text: props.current.code,
      mimeType: 'text/plain',
    });
  }
}
</script>

<template>
  <div v-if="lg" class="grid gap-2" :class="desktop ? 'grid-cols-6' : 'grid-cols-5'">
    <ActionTooltip :label="t('execute')">
      <Button
        variant="ghost"
        size="icon"
        :disabled="loading || !current?.id"
        @click="onExecute"
      >
        <PlayIcon stroke-width="1.5px" />
      </Button>
    </ActionTooltip>

    <ActionTooltip :label="t('save')">
      <Button
        variant="ghost"
        size="icon"
        :disabled="loading || !current"
        @click="onSave"
      >
        <SaveIcon stroke-width="1.5px" />
      </Button>
    </ActionTooltip>

    <ActionTooltip :label="t('remove')">
      <Button
        variant="ghost"
        size="icon"
        :disabled="loading || !current"
        @click="onRemove"
      >
        <Trash2Icon stroke-width="1.5px" />
      </Button>
    </ActionTooltip>

    <ActionTooltip v-if="desktop" :label="t('import')">
      <Button
        variant="ghost"
        size="icon"
        :disabled="loading"
        @click="onImport"
      >
        <FileDownIcon stroke-width="1.5px" />
      </Button>
    </ActionTooltip>

    <ActionTooltip v-if="desktop" :label="t('export')">
      <Button
        variant="ghost"
        size="icon"
        :disabled="loading || !current"
        @click="onExport"
      >
        <FileUpIcon stroke-width="1.5px" />
      </Button>
    </ActionTooltip>

    <ActionTooltip v-if="mobile" :label="t('share')">
      <Button
        variant="ghost"
        size="icon"
        :disabled="loading || !current?.code"
        @click="share"
      >
        <Share2Icon stroke-width="1.5px" />
      </Button>
    </ActionTooltip>

    <ActionTooltip :label="t('registry')">
      <Button
        variant="ghost"
        size="icon"
        :disabled="loading"
        role="link"
        tabindex="0"
        @click="() => go('nsr')"
      >
        <GlobeIcon stroke-width="1.5px" />
      </Button>
    </ActionTooltip>
  </div>

  <template v-else>
    <ScriptDialog
      v-model="isScriptDialogOpen"
      :scripts
      :loading
      @script-click="onScriptClick"
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
          <DropdownMenuItem :disabled="loading" @click="onCreate">
            {{ t('create') }}
          </DropdownMenuItem>

          <DropdownMenuItem
            :disabled="loading || isScriptDialogOpen || scripts.length === 0"
            @click="toggleScriptDialog"
          >
            {{ t('select') }}
          </DropdownMenuItem>

          <DropdownMenuItem :disabled="loading || !current?.id" @click="onExecute">
            {{ t('execute') }}
          </DropdownMenuItem>

          <DropdownMenuItem :disabled="loading || !current" @click="onSave">
            {{ t('save') }}
          </DropdownMenuItem>

          <DropdownMenuItem :disabled="loading || !current" @click="onRemove">
            {{ t('remove') }}
          </DropdownMenuItem>

          <DropdownMenuItem v-if="desktop" :disabled="loading" @click="onImport">
            {{ t('import') }}
          </DropdownMenuItem>

          <DropdownMenuItem v-if="desktop" :disabled="loading || !current" @click="onExport">
            {{ t('export') }}
          </DropdownMenuItem>

          <DropdownMenuItem v-if="mobile" :disabled="loading || !current?.code" @click="share">
            {{ t('share') }}
          </DropdownMenuItem>

          <DropdownMenuItem :disabled="loading">
            <RouterLink :to="{ name: 'nsr' satisfies ScriptScene }" class="w-full cursor-default">
              <span>{{ t('registry') }}</span>
            </RouterLink>
          </DropdownMenuItem>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  </template>
</template>
