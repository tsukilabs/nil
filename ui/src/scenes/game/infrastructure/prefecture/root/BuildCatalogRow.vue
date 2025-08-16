<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { storeToRefs } from 'pinia';
import { computed, nextTick } from 'vue';
import { MenuIcon } from 'lucide-vue-next';
import type { ResourcesImpl } from '@/core/model/resources';
import BuildCatalogBuilding from './BuildCatalogBuilding.vue';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import { usePrefectureSettings } from '@/settings/infrastructure/prefecture';
import enUS from '@/locale/en-US/scenes/game/infrastructure/prefecture.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/prefecture.json';
import type { BuildingImpl } from '@/core/model/infrastructure/building/abstract';
import type { PrefectureImpl } from '@/core/model/infrastructure/building/prefecture/prefecture';
import { useResolvedBuildingLevel } from '@/composables/infrastructure/useResolvedBuildingLevel';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
  TableCell,
  TableRow,
} from '@tb-dev/vue-components';

const props = defineProps<{
  entry: PrefectureBuildCatalogEntry;
  building: BuildingImpl;
  prefecture: PrefectureImpl;
  scene: GameScene;
  loading: boolean;
  isPlayerTurn: boolean;
  playerResources: Option<ResourcesImpl>;
  onBuildOrder: (kind: PrefectureBuildOrderKind) => void;
  onToggle: () => void;
}>();

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const { player } = NIL.player.refs();

const settings = usePrefectureSettings();
const { hideMaxed, hideUnmet } = storeToRefs(settings);

const { md } = useBreakpoints();
const level = useResolvedBuildingLevel(() => props.building);

const canBuild = computed(() => {
  if (
    !props.loading &&
    player.value &&
    props.isPlayerTurn &&
    props.prefecture.enabled &&
    level.value.next <= level.value.max &&
    props.entry.kind === 'available'
  ) {
    return player.value.hasResources(props.entry.recipe.resources);
  }

  return false;
});

const canDemolish = computed(() => {
  return (
    !props.loading &&
    player.value &&
    props.isPlayerTurn &&
    props.prefecture.enabled &&
    level.value.previous >= level.value.min
  );
});

async function makeOrder(kind: PrefectureBuildOrderKind) {
  await nextTick();
  if ((kind === 'construction' && canBuild.value) || (kind === 'demolition' && canDemolish.value)) {
    props.onBuildOrder(kind);
  }
}
</script>

<template>
  <TableRow
    v-if="entry.kind === 'available'"
    @dblclick="() => makeOrder('construction')"
  >
    <TableCell>
      <BuildCatalogBuilding :building :scene />
    </TableCell>

    <TableCell>
      <div class="grid grid-cols-5 items-center justify-start gap-4">
        <Wood :amount="entry.recipe.resources.wood" :limit="playerResources?.wood" />
        <Stone :amount="entry.recipe.resources.stone" :limit="playerResources?.stone" />
        <Iron :amount="entry.recipe.resources.iron" :limit="playerResources?.iron" />
        <Food :amount="entry.recipe.maintenance" />
        <Workforce :amount="entry.recipe.workforce" />
      </div>
    </TableCell>

    <TableCell>
      <div v-if="md" class="grid max-w-fit grid-cols-3 items-center justify-start gap-4">
        <Button
          variant="default"
          size="sm"
          :disabled="!canBuild"
          class="max-w-24"
          @click="() => makeOrder('construction')"
        >
          <span>{{ t('build') }}</span>
        </Button>
        <Button
          variant="secondary"
          size="sm"
          :disabled="loading || !isPlayerTurn"
          class="max-w-24"
          @click="() => onToggle()"
        >
          <span>{{ building.enabled ? t('disable') : t('enable') }}</span>
        </Button>
        <Button
          variant="destructive"
          size="sm"
          :disabled="!canDemolish"
          class="max-w-24"
          @click="() => makeOrder('demolition')"
        >
          <span>{{ t('demolish') }}</span>
        </Button>
      </div>

      <DropdownMenu v-else>
        <DropdownMenuTrigger as-child>
          <Button variant="ghost" size="icon">
            <MenuIcon stroke-width="1.5px" />
          </Button>
        </DropdownMenuTrigger>

        <DropdownMenuContent
          align="end"
          :align-offset="-15"
          side="bottom"
          :side-offset="5"
          class="w-56"
        >
          <DropdownMenuGroup>
            <DropdownMenuItem :disabled="!canBuild" @click="() => makeOrder('construction')">
              {{ t('build') }}
            </DropdownMenuItem>

            <DropdownMenuItem :disabled="loading || !isPlayerTurn" @click="() => onToggle()">
              {{ building.enabled ? t('disable') : t('enable') }}
            </DropdownMenuItem>

            <DropdownMenuItem :disabled="!canDemolish" @click="() => makeOrder('demolition')">
              {{ t('demolish') }}
            </DropdownMenuItem>
          </DropdownMenuGroup>
        </DropdownMenuContent>
      </DropdownMenu>
    </TableCell>
  </TableRow>

  <TableRow v-else-if="(entry.kind === 'maxed' && !hideMaxed) || (entry.kind === 'unmet' && !hideUnmet)">
    <TableCell>
      <BuildCatalogBuilding :building :scene />
    </TableCell>

    <TableCell colspan="4">
      <div class="text-muted-foreground flex w-full items-center justify-center text-sm">
        <span v-if="entry.kind === 'maxed'">
          {{ t('building-fully-constructed') }}
        </span>
        <span v-else-if="entry.kind === 'unmet'">
          {{ t('not-yet-available') }}
        </span>
      </div>
    </TableCell>
  </TableRow>

  <TableRow v-else class="hidden" />
</template>
