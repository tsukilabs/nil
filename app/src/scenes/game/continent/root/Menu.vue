<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { MenuIcon } from 'lucide-vue-next';
import { useBreakpoints } from '@tb-dev/vue';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@tb-dev/vue-components';

const { t } = useI18n();

const { md } = useBreakpoints();
</script>

<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <Button variant="default" size="icon" class="absolute z-20 top-5 right-5">
        <MenuIcon stroke-width="1.5px" />
      </Button>
    </DropdownMenuTrigger>

    <DropdownMenuContent
      align="end"
      :align-offset="-15"
      side="bottom"
      :side-offset="md ? 5 : 0"
      class="w-40"
    >
      <DropdownMenuGroup>
        <DropdownMenuItem>
          <RouterLink :to="{ name: 'continent-cities' satisfies ContinentScene }" class="w-full">
            {{ t('city', 2) }}
          </RouterLink>
        </DropdownMenuItem>
      </DropdownMenuGroup>
    </DropdownMenuContent>
  </DropdownMenu>
</template>
