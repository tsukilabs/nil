<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { Badge } from '@ui/badge';
import { useI18n } from 'vue-i18n';
import type { BattleWinner } from '@/types/core/battle';
import { CoordImpl } from '@/core/model/continent/coord';
import enUS from '@/locale/en-US/scenes/game/report.json';
import ptBR from '@/locale/pt-BR/scenes/game/report.json';
import { Table, TableBody, TableCell, TableHead, TableRow } from '@ui/table';
import { useCityOwnerSceneLink } from '@/composables/city/useCityOwnerSceneLink';
import { useCityProfileSceneLink } from '@/composables/city/useCityProfileSceneLink';
import BattleReportPersonnel from '@/scenes/game/report/view/BattleReportPersonnel.vue';

const props = defineProps<{
  kind: BattleWinner;
  winner: BattleWinner;
  ruler: Ruler;
  personnel: ArmyPersonnel;
  losses: ArmyPersonnel;
  city: PublicCity;
}>();

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const toRulerProfile = useCityOwnerSceneLink(() => props.ruler);
const toCityProfile = useCityProfileSceneLink(() => props.city);
</script>

<template>
  <Table class="max-w-[800px]">
    <TableBody>
      <TableRow>
        <TableHead>{{ t(kind) }}</TableHead>
        <TableCell>
          <div class="flex items-center gap-4">
            <RouterLink v-if="toRulerProfile" :to="toRulerProfile">
              {{ ruler.id }}
            </RouterLink>

            <Badge v-if="kind === winner">
              <span class="text-xs">{{ t('winner') }}</span>
            </Badge>
          </div>
        </TableCell>
      </TableRow>

      <TableRow>
        <TableHead>{{ kind === 'attacker' ? t('origin') : t('destination') }}</TableHead>
        <TableCell>
          <RouterLink v-if="toCityProfile" :to="toCityProfile">
            {{ `${city.name} (${CoordImpl.format(city.coord)})` }}
          </RouterLink>
        </TableCell>
      </TableRow>

      <TableRow class="hover:bg-card">
        <TableCell colspan="2">
          <BattleReportPersonnel :personnel :losses />
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
