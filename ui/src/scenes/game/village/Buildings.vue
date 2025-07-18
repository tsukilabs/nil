<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { VillageImpl } from '@/core/model/village';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  Table,
  TableBody,
  TableCell,
  TableRow,
} from '@tb-dev/vue-components';

defineProps<{ village: VillageImpl }>();

const { t } = useI18n();
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>
        <span>{{ t('building', 2) }}</span>
      </CardTitle>
    </CardHeader>

    <CardContent class="px-2 py-0">
      <Table>
        <TableBody>
          <TableRow>
            <TableCell>
              <RouterLink
                :to="{ name: 'prefecture' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('prefecture') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${village.prefecture.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="village.academy.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'academy' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('academy') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${village.academy.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="village.stable.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'stable' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('stable') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${village.stable.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="village.sawmill.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'sawmill' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('sawmill') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${village.sawmill.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="village.quarry.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'quarry' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('quarry') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${village.quarry.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="village.ironMine.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'iron-mine' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('iron-mine') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${village.ironMine.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="village.farm.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'farm' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('farm') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${village.farm.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="village.warehouse.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'warehouse' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('warehouse') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${village.warehouse.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="village.silo.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'silo' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('silo') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${village.silo.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="village.wall.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'wall' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('wall') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${village.wall.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </CardContent>
  </Card>
</template>
