<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useToggle } from '@vueuse/core';
import ScriptDialog from './ScriptDialog.vue';
import ActionTooltip from './ActionTooltip.vue';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import { FileDownIcon, FileUpIcon, GlobeIcon, MenuIcon, PlayIcon, SaveIcon, Trash2Icon } from 'lucide-vue-next';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@tb-dev/vue-components';

defineProps<{
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
</script>

<template>
  <div v-if="lg" class="grid grid-cols-6 gap-2">
    <ActionTooltip :label="t('execute')">
      <Button variant="ghost" size="icon" :disabled="loading || !current?.id" @click="onExecute">
        <PlayIcon />
      </Button>
    </ActionTooltip>

    <ActionTooltip :label="t('save')">
      <Button variant="ghost" size="icon" :disabled="loading || !current" @click="onSave">
        <SaveIcon />
      </Button>
    </ActionTooltip>

    <ActionTooltip :label="t('remove')">
      <Button variant="ghost" size="icon" :disabled="loading || !current" @click="onRemove">
        <Trash2Icon />
      </Button>
    </ActionTooltip>

    <ActionTooltip :label="t('import')">
      <Button variant="ghost" size="icon" :disabled="loading" @click="onImport">
        <FileDownIcon />
      </Button>
    </ActionTooltip>

    <ActionTooltip :label="t('export')">
      <Button variant="ghost" size="icon" :disabled="loading || !current" @click="onExport">
        <FileUpIcon />
      </Button>
    </ActionTooltip>

    <ActionTooltip :label="t('nsr')">
      <Button variant="ghost" size="icon" :disabled="loading">
        <RouterLink :to="{ name: 'nsr' satisfies ScriptScene }">
          <GlobeIcon />
        </RouterLink>
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
          <MenuIcon />
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

          <DropdownMenuItem :disabled="loading || isScriptDialogOpen" @click="toggleScriptDialog">
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

          <DropdownMenuItem :disabled="loading" @click="onImport">
            {{ t('import') }}
          </DropdownMenuItem>

          <DropdownMenuItem :disabled="loading || !current" @click="onExport">
            {{ t('export') }}
          </DropdownMenuItem>

          <DropdownMenuItem :disabled="loading">
            <RouterLink :to="{ name: 'nsr' satisfies ScriptScene }" class="w-full cursor-default">
              <span>{{ t('nsr') }}</span>
            </RouterLink>
          </DropdownMenuItem>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  </template>
</template>
