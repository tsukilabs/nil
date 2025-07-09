<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { VillageImpl } from '@/core/model/village';
import { Card, Table, TableLink, TableRow } from '@tb-dev/vue-components';

defineProps<{ village: VillageImpl }>();

const { t } = useI18n();
</script>

<template>
  <Card content-class="px-2">
    <template #title>
      <span>{{ t('building', 2) }}</span>
    </template>

    <Table>
      <TableRow>
        <TableLink to="prefecture" link-class="gap-2 p-1">
          <span>{{ t('prefecture') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.prefecture.level})` }}
          </span>
        </TableLink>
      </TableRow>

      <TableRow v-if="village.academy.level > 0">
        <TableLink to="academy" link-class="gap-2 p-1">
          <span>{{ t('academy') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.academy.level})` }}
          </span>
        </TableLink>
      </TableRow>

      <TableRow v-if="village.stable.level > 0">
        <TableLink to="stable" link-class="gap-2 p-1">
          <span>{{ t('stable') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.stable.level})` }}
          </span>
        </TableLink>
      </TableRow>

      <TableRow v-if="village.sawmill.level > 0">
        <TableLink to="sawmill" link-class="gap-2 p-1">
          <span>{{ t('sawmill') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.sawmill.level})` }}
          </span>
        </TableLink>
      </TableRow>

      <TableRow v-if="village.quarry.level > 0">
        <TableLink to="quarry" link-class="gap-2 p-1">
          <span>{{ t('quarry') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.quarry.level})` }}
          </span>
        </TableLink>
      </TableRow>

      <TableRow v-if="village.ironMine.level > 0">
        <TableLink to="iron-mine" link-class="gap-2 p-1">
          <span>{{ t('iron-mine') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.ironMine.level})` }}
          </span>
        </TableLink>
      </TableRow>

      <TableRow v-if="village.farm.level > 0">
        <TableLink to="farm" link-class="gap-2 p-1">
          <span>{{ t('farm') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.farm.level})` }}
          </span>
        </TableLink>
      </TableRow>

      <TableRow v-if="village.warehouse.level > 0">
        <TableLink to="warehouse" link-class="gap-2 p-1">
          <span>{{ t('warehouse') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.warehouse.level})` }}
          </span>
        </TableLink>
      </TableRow>

      <TableRow v-if="village.silo.level > 0">
        <TableLink to="silo" link-class="gap-2 p-1">
          <span>{{ t('silo') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.silo.level})` }}
          </span>
        </TableLink>
      </TableRow>

      <TableRow v-if="village.wall.level > 0">
        <TableLink to="wall" link-class="gap-2 p-1">
          <span>{{ t('wall') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.wall.level})` }}
          </span>
        </TableLink>
      </TableRow>
    </Table>
  </Card>
</template>
