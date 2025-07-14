<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { compare } from '@/lib/intl';
import {
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from '@tb-dev/vue-components';

const open = defineModel<boolean>('open', { required: true });

const { t } = useI18n();

interface FinderItem {
  value: GameScene;
  label: string;
}

const items = computed<FinderItem[]>(() => {
  const _items: FinderItem[] = [
    {
      value: 'academy',
      label: t('academy'),
    },
    {
      value: 'continent',
      label: t('continent-map'),
    },
    {
      value: 'farm',
      label: t('farm'),
    },
    {
      value: 'iron-mine',
      label: t('iron-mine'),
    },
    {
      value: 'prefecture',
      label: t('prefecture'),
    },
    {
      value: 'quarry',
      label: t('quarry'),
    },
    {
      value: 'sawmill',
      label: t('sawmill'),
    },
    {
      value: 'silo',
      label: t('silo'),
    },
    {
      value: 'stable',
      label: t('stable'),
    },
    {
      value: 'script',
      label: t('script', 2),
    },
    {
      value: 'village',
      label: t('village'),
    },
    {
      value: 'village-management',
      label: t('village-management'),
    },
    {
      value: 'wall',
      label: t('wall'),
    },
    {
      value: 'warehouse',
      label: t('warehouse'),
    },
  ];

  _items.sort((a, b) => compare(a.label, b.label));

  return _items;
});
</script>

<template>
  <CommandDialog v-model:open="open">
    <CommandInput :placeholder="t('search')" />
    <CommandList>
      <CommandEmpty>{{ t('no-results-found') }}</CommandEmpty>
      <CommandGroup>
        <CommandItem v-for="item of items" :key="item.value" :value="item.value">
          <RouterLink :to="{ name: item.value }" class="w-full" @click="() => (open = false)">
            {{ item.label }}
          </RouterLink>
        </CommandItem>
      </CommandGroup>
    </CommandList>
  </CommandDialog>
</template>
