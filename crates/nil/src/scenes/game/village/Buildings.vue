<script setup lang="ts">
import { useLocale } from '@/locale';
import type { VillageImpl } from '@/core/village';
import { Card, Table, TableCellLink, TableRow } from '@/components';

interface Props {
  village: VillageImpl;
}

defineProps<Props>();

const { t } = useLocale();
</script>

<template>
  <Card content-class="p-4 pb-8">
    <Table>
      <TableRow>
        <TableCellLink to="prefecture" link-class="gap-2 p-1">
          <span>{{ t('prefecture') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.prefecture.level})` }}
          </span>
        </TableCellLink>
      </TableRow>
      <TableRow v-if="village.academy.level > 0">
        <TableCellLink to="academy" link-class="gap-2 p-1">
          <span>{{ t('academy') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.academy.level})` }}
          </span>
        </TableCellLink>
      </TableRow>
      <TableRow v-if="village.stable.level > 0">
        <TableCellLink to="stable" link-class="gap-2 p-1">
          <span>{{ t('stable') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.stable.level})` }}
          </span>
        </TableCellLink>
      </TableRow>
      <TableRow v-if="village.sawmill.level > 0">
        <TableCellLink to="sawmill" link-class="gap-2 p-1">
          <span>{{ t('sawmill') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.sawmill.level})` }}
          </span>
        </TableCellLink>
      </TableRow>
      <TableRow v-if="village.quarry.level > 0">
        <TableCellLink to="quarry" link-class="gap-2 p-1">
          <span>{{ t('quarry') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.quarry.level})` }}
          </span>
        </TableCellLink>
      </TableRow>
      <TableRow v-if="village.ironMine.level > 0">
        <TableCellLink to="iron-mine" link-class="gap-2 p-1">
          <span>{{ t('iron-mine') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.ironMine.level})` }}
          </span>
        </TableCellLink>
      </TableRow>
      <TableRow v-if="village.farm.level > 0">
        <TableCellLink to="farm" link-class="gap-2 p-1">
          <span>{{ t('farm') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.farm.level})` }}
          </span>
        </TableCellLink>
      </TableRow>
      <TableRow v-if="village.warehouse.level > 0">
        <TableCellLink to="warehouse" link-class="gap-2 p-1">
          <span>{{ t('warehouse') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.warehouse.level})` }}
          </span>
        </TableCellLink>
      </TableRow>
      <TableRow v-if="village.silo.level > 0">
        <TableCellLink to="silo" link-class="gap-2 p-1">
          <span>{{ t('silo') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.silo.level})` }}
          </span>
        </TableCellLink>
      </TableRow>
      <TableRow v-if="village.wall.level > 0">
        <TableCellLink to="wall" link-class="gap-2 p-1">
          <span>{{ t('wall') }}</span>
          <span class="text-muted-foreground">
            {{ `(${t('level')} ${village.wall.level})` }}
          </span>
        </TableCellLink>
      </TableRow>
    </Table>
  </Card>
</template>
