<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { PLACEHOLDER } from '@/lib/string';
import { compare, formatInt } from '@/lib/intl';
import { computed, onBeforeMount, ref, shallowRef } from 'vue';
import type { PublicCityImpl } from '@/core/model/city/public-city';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@tb-dev/vue-components';

type SortMode = 'coord' | 'distance' | 'name' | 'owner' | 'score';
type SortOrder = 'asc' | 'desc';

interface Props {
  cities: readonly PublicCityImpl[];
  showOwner?: boolean;
  sortable?: boolean;
  defaultSortMode?: SortMode;
  defaultSortOrder?: SortOrder;
  getDistance?: (coord: Coord) => Option<number>;
}

const props = withDefaults(defineProps<Props>(), {
  defaultSortMode: 'name',
  defaultSortOrder: 'asc',
});

const { t } = useI18n();

const sorted = shallowRef<readonly PublicCityImpl[]>([]);
const sortMode = ref<SortMode>('name');
const sortOrder = ref<SortOrder>('asc');

const tableHeadClass = computed(() => {
  return props.sortable ? 'cursor-pointer' : null;
});

onBeforeMount(() => {
  sort(props.defaultSortMode, props.defaultSortOrder !== 'desc');
});

function sort(mode: SortMode, ascending: boolean) {
  if (props.sortable) {
    if (mode === sortMode.value) {
      sortOrder.value = ascending ? 'asc' : 'desc';
    }
    else {
      sortOrder.value = 'asc';
    }

    sortMode.value = mode;

    sorted.value = props.cities.toSorted((a, b) => {
      switch (mode) {
        case 'coord': {
          if (a.coord.x === b.coord.y) {
            return sortOrder.value === 'asc' ?
              a.coord.y - b.coord.y :
              b.coord.y - a.coord.y;
          }
          else {
            return sortOrder.value === 'asc' ?
              a.coord.x - b.coord.x :
              b.coord.x - a.coord.x;
          }
        }
        case 'distance': {
          if (props.getDistance) {
            const distanceA = props.getDistance(a.coord) ?? 0;
            const distanceB = props.getDistance(b.coord) ?? 0;
            return sortOrder.value === 'asc' ?
              distanceA - distanceB :
              distanceB - distanceA;
          }
          else {
            return 0;
          }
        }
        case 'owner': {
          return sortOrder.value === 'asc' ?
            compare(a.owner.id, b.owner.id) :
            compare(b.owner.id, a.owner.id);
        }
        case 'score': {
          return sortOrder.value === 'asc' ?
            a.score - b.score :
            b.score - a.score;
        }
        case 'name':
        default: {
          return sortOrder.value === 'asc' ?
            compare(a.name, b.name) :
            compare(b.name, a.name);
        }
      }
    });
  }
}

defineExpose({
  sort(mode?: Option<SortMode>, ascending?: Option<boolean>) {
    sort(mode ?? sortMode.value, ascending ?? sortOrder.value === 'asc');
  },
});
</script>

<template>
  <Table>
    <TableHeader>
      <TableRow class="hover:bg-card">
        <TableHead
          :class="tableHeadClass"
          @click="() => sort('name', sortOrder === 'asc' ? false : true)"
        >
          {{ t('city', 2) }}
        </TableHead>

        <TableHead
          :class="tableHeadClass"
          @click="() => sort('coord', sortOrder === 'asc' ? false : true)"
        >
          {{ t('coordinate', 2) }}
        </TableHead>

        <TableHead
          :class="tableHeadClass"
          @click="() => sort('score', sortOrder === 'asc' ? false : true)"
        >
          {{ t('point', 2) }}
        </TableHead>

        <TableHead
          v-if="getDistance"
          :class="tableHeadClass"
          @click="() => sort('distance', sortOrder === 'asc' ? false : true)"
        >
          {{ t('distance') }}
        </TableHead>

        <TableHead
          v-if="showOwner"
          :class="tableHeadClass"
          @click="() => sort('owner', sortOrder === 'asc' ? false : true)"
        >
          {{ t('owner') }}
        </TableHead>
      </TableRow>
    </TableHeader>

    <TableBody>
      <TableRow
        v-for="city of sorted"
        :key="city.index"
        role="link"
        tabindex="0"
        class="cursor-pointer"
        @click="() => city.goToProfile()"
        @keydown.enter="() => city.goToProfile()"
      >
        <TableCell>
          {{ city.name }}
        </TableCell>

        <TableCell>
          {{ city.coord.format() }}
        </TableCell>

        <TableCell>
          {{ formatInt(city.score) }}
        </TableCell>

        <TableCell v-if="getDistance">
          {{ getDistance(city.coord) ?? PLACEHOLDER }}
        </TableCell>

        <TableCell
          v-if="showOwner"
          role="link"
          tabindex="0"
          class="cursor-pointer"
          @click.stop.prevent="() => city.goToOwnerProfile()"
          @keydown.enter.stop.prevent="() => city.goToOwnerProfile()"
        >
          {{ city.owner.id }}
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
