<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { MenuIcon } from 'lucide-vue-next';
import { useBreakpoints } from '@tb-dev/vue';
import type { ScriptImpl } from '@/core/model/scripts/script';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@tb-dev/vue-components';

defineProps<{
  currentScript: Option<ScriptImpl>;
  disabled: boolean;
  toggleSheet: (value?: boolean) => void;
  onExecute: () => Promise<void>;
  onRemove: () => Promise<void>;
}>();

const { t } = useI18n();

const { md } = useBreakpoints();
</script>

<template>
  <div class="flex gap-4">
    <div>
      <Button
        variant="default"
        size="sm"
        :disabled="!currentScript || disabled"
        @click="onExecute"
      >
        <span>{{ t('execute') }}</span>
      </Button>
    </div>

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
        :side-offset="md ? 5 : 0"
        class="w-56"
      >
        <template v-if="!md">
          <DropdownMenuGroup>
            <DropdownMenuItem @click="() => toggleSheet(true)">
              <span>{{ t('script', 2) }}</span>
            </DropdownMenuItem>
          </DropdownMenuGroup>

          <DropdownMenuSeparator />
        </template>

        <DropdownMenuGroup>
          <DropdownMenuItem :disabled="!currentScript || disabled" @click="onRemove">
            <span>{{ t('remove') }}</span>
          </DropdownMenuItem>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  </div>
</template>
