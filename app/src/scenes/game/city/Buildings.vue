<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { CityImpl } from '@/core/model/city/city';
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

defineProps<{ city: CityImpl; }>();

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
                  {{ `(${t('level')} ${city.prefecture.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="city.academy.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'academy' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('academy') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${city.academy.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="city.stable.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'stable' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('stable') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${city.stable.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="city.sawmill.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'sawmill' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('sawmill') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${city.sawmill.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="city.quarry.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'quarry' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('quarry') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${city.quarry.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="city.ironMine.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'iron-mine' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('iron-mine') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${city.ironMine.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="city.farm.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'farm' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('farm') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${city.farm.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="city.warehouse.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'warehouse' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('warehouse') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${city.warehouse.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="city.silo.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'silo' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('silo') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${city.silo.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>

          <TableRow v-if="city.wall.level > 0">
            <TableCell>
              <RouterLink
                :to="{ name: 'wall' satisfies InfrastructureScene }"
                class="flex w-full gap-2 p-1"
              >
                <span>{{ t('wall') }}</span>
                <span class="text-muted-foreground">
                  {{ `(${t('level')} ${city.wall.level})` }}
                </span>
              </RouterLink>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </CardContent>
  </Card>
</template>
