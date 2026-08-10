<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import { cn } from "@ui/utils";
import { Button } from "@ui/button";
import { useBreakpoints } from "@tb-dev/vue";
import { useI18n } from "@tsukilabs/nil-i18n";
import type { MaybePromise } from "@tb-dev/utils";
import type { PrefectureBuildOrderKind } from "@tsukilabs/nil-bindings";
import type { BuildingImpl } from "@/core/model/infrastructure/building/abstract";

const props = defineProps<{
  building: BuildingImpl;
  canBuild: boolean;
  canDemolish: boolean;
  isPlayerTurn: boolean;
  loading: boolean;
  class?: string;
  onOrder: (kind: PrefectureBuildOrderKind) => MaybePromise<void>;
  onToggle: () => void;
}>();

const { t } = useI18n();

const { sm } = useBreakpoints();
</script>

<template>
  <div :class="cn('grid max-w-fit grid-cols-3 items-center justify-start gap-4', props.class)">
    <Button
      variant="default"
      :size="sm ? 'sm' : 'xs'"
      :disabled="!canBuild"
      class="max-w-32"
      @click="() => onOrder('construction')"
    >
      <span>{{ t("build") }}</span>
    </Button>
    <Button
      variant="secondary"
      :size="sm ? 'sm' : 'xs'"
      :disabled="loading || !isPlayerTurn"
      class="max-w-32"
      @click="() => onToggle()"
    >
      <span>{{ building.enabled ? t("disable") : t("enable") }}</span>
    </Button>
    <Button
      variant="destructive"
      :size="sm ? 'sm' : 'xs'"
      :disabled="!canDemolish"
      class="max-w-32"
      @click="() => onOrder('demolition')"
    >
      <span>{{ t("prefecture.demolish") }}</span>
    </Button>
  </div>
</template>
