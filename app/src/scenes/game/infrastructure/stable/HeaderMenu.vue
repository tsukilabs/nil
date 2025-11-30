<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router';
import { MenuIcon } from 'lucide-vue-next';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import {
  Button,
  type ButtonVariant,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@tb-dev/vue-components';

const { t } = useI18n();

const route = useRoute();
const { md } = useBreakpoints();

function getButtonVariant(scene: StableScene): ButtonVariant {
  return route.name === scene ? 'secondary' : 'ghost';
}
</script>

<template>
  <div v-if="md" class="grid grid-cols-2 gap-2">
    <Button
      size="sm"
      :variant="getButtonVariant('stable')"
      role="link"
      tabindex="0"
      @click="() => go('stable')"
    >
      <span>{{ t('construction') }}</span>
    </Button>

    <Button
      size="sm"
      :variant="getButtonVariant('stable-settings')"
      role="link"
      tabindex="0"
      @click="() => go('stable-settings')"
    >
      <span>{{ t('settings') }}</span>
    </Button>
  </div>

  <DropdownMenu v-else>
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
      <DropdownMenuGroup>
        <DropdownMenuItem>
          <RouterLink :to="{ name: 'stable' satisfies StableScene }" class="w-full">
            {{ t('construction') }}
          </RouterLink>
        </DropdownMenuItem>
        <DropdownMenuItem>
          <RouterLink :to="{ name: 'stable-settings' satisfies StableScene }" class="w-full">
            {{ t('settings') }}
          </RouterLink>
        </DropdownMenuItem>
      </DropdownMenuGroup>
    </DropdownMenuContent>
  </DropdownMenu>
</template>
