<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router';
import { MenuIcon } from 'lucide-vue-next';
import { useBreakpoints } from '@tb-dev/vue';
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

function getButtonVariant(scene: WarRoomScene): ButtonVariant {
  return route.name === scene ? 'secondary' : 'ghost';
}
</script>

<template>
  <div v-if="md" class="grid grid-cols-2 gap-2">
    <Button
      size="sm"
      :variant="getButtonVariant('war-room')"
      role="link"
      tabindex="0"
      @click="() => go('war-room')"
    >
      <span>{{ t('maneuver', 2) }}</span>
    </Button>

    <Button
      size="sm"
      :variant="getButtonVariant('war-room-simulator')"
      role="link"
      tabindex="0"
      @click="() => go('war-room-simulator')"
    >
      <span>{{ t('simulator') }}</span>
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
          <RouterLink :to="{ name: 'war-room' satisfies WarRoomScene }" class="w-full">
            {{ t('maneuver', 2) }}
          </RouterLink>
        </DropdownMenuItem>
        <DropdownMenuItem>
          <RouterLink :to="{ name: 'war-room-simulator' satisfies WarRoomScene }" class="w-full">
            {{ t('simulator') }}
          </RouterLink>
        </DropdownMenuItem>
      </DropdownMenuGroup>
    </DropdownMenuContent>
  </DropdownMenu>
</template>
