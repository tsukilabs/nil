<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { compare } from '@/lib/intl';
import { computed, nextTick } from 'vue';
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
  value: GameScene | 'settings';
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
      value: 'nsr',
      label: 'NSR',
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
      value: 'settings',
      label: t('settings'),
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

async function onClick() {
  await nextTick();
  open.value = false;
}
</script>

<template>
  <CommandDialog v-model:open="open">
    <CommandInput :placeholder="t('search')" />
    <CommandList>
      <CommandEmpty>{{ t('no-results-found') }}</CommandEmpty>
      <CommandGroup>
        <CommandItem v-for="item of items" :key="item.value" :value="item.value" as-child>
          <RouterLink :to="{ name: item.value }" class="w-full cursor-pointer" @click="onClick">
            {{ item.label }}
          </RouterLink>
        </CommandItem>
      </CommandGroup>
    </CommandList>
  </CommandDialog>
</template>
