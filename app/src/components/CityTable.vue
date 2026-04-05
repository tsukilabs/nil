<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { computed, ref } from 'vue';
import { compare } from '@/lib/intl';
import { PLACEHOLDER } from '@/lib/string';
import type { PublicCityImpl } from '@/core/model/city/public-city';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@ui/table';

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

const sortMode = ref<Option<SortMode>>();
const sortOrder = ref<Option<SortOrder>>();

const sorted = computed<readonly PublicCityImpl[]>(() => {
  if (props.sortable) {
    const mode = sortMode.value ?? props.defaultSortMode;
    const order = sortOrder.value ?? props.defaultSortOrder;

    return props.cities.toSorted((a, b) => {
      switch (mode) {
        case 'coord': {
          if (a.coord.x === b.coord.y) {
            return order === 'asc' ?
              a.coord.y - b.coord.y :
              b.coord.y - a.coord.y;
          }
          else {
            return order === 'asc' ?
              a.coord.x - b.coord.x :
              b.coord.x - a.coord.x;
          }
        }
        case 'distance': {
          if (props.getDistance) {
            const distanceA = props.getDistance(a.coord) ?? 0;
            const distanceB = props.getDistance(b.coord) ?? 0;
            return order === 'asc' ?
              distanceA - distanceB :
              distanceB - distanceA;
          }
          else {
            return 0;
          }
        }
        case 'name': {
          return order === 'asc' ?
            compare(a.name, b.name) :
            compare(b.name, a.name);
        }
        case 'owner': {
          return order === 'asc' ?
            compare(a.owner.id, b.owner.id) :
            compare(b.owner.id, a.owner.id);
        }
        case 'score': {
          return order === 'asc' ?
            a.score - b.score :
            b.score - a.score;
        }
        default:
          return 0;
      }
    });
  }

  return props.cities;
});

const tableHeadClass = computed(() => {
  return props.sortable ? 'cursor-pointer' : null;
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
  }
}
</script>

<template>
  <Table>
    <TableHeader>
      <TableRow class="hover:bg-card">
        <TableHead
          :class="tableHeadClass"
          @click="() => sort('name', sortOrder === 'asc' ? false : true)"
        >
          {{ t('city') }}
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
          {{ city.score }}
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
