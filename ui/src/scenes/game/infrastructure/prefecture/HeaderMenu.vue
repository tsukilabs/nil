<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router';
import { MenuIcon } from 'lucide-vue-next';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import enUS from '@/locale/en-US/scenes/game/infrastructure/prefecture.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/prefecture.json';
import {
  Button,
  type ButtonVariant,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const route = useRoute();
const { md } = useBreakpoints();

function getButtonVariant(scene: PrefectureScene): ButtonVariant {
  return route.name === scene ? 'secondary' : 'ghost';
}
</script>

<template>
  <div v-if="md" class="grid grid-cols-2 gap-2">
    <Button size="sm" :variant="getButtonVariant('prefecture')">
      <RouterLink :to="{ name: 'prefecture' satisfies PrefectureScene }">
        {{ t('construction') }}
      </RouterLink>
    </Button>
    <Button size="sm" :variant="getButtonVariant('prefecture-settings')">
      <RouterLink :to="{ name: 'prefecture-settings' satisfies PrefectureScene }">
        {{ t('settings') }}
      </RouterLink>
    </Button>
  </div>

  <DropdownMenu v-else>
    <DropdownMenuTrigger as-child>
      <Button variant="ghost" size="icon">
        <MenuIcon />
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
          <RouterLink :to="{ name: 'prefecture' satisfies PrefectureScene }" class="w-full">
            {{ t('construction') }}
          </RouterLink>
        </DropdownMenuItem>
        <DropdownMenuItem>
          <RouterLink :to="{ name: 'prefecture-settings' satisfies PrefectureScene }" class="w-full">
            {{ t('settings') }}
          </RouterLink>
        </DropdownMenuItem>
      </DropdownMenuGroup>
    </DropdownMenuContent>
  </DropdownMenu>
</template>
