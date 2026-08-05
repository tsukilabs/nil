<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import { computed } from "vue";
import { Label } from "@ui/label";
import { Slider } from "@ui/slider";
import { CONSTS } from "@/lib/global";
import { formatPercent } from "@/lib/intl";
import { useI18n } from "@tsukilabs/nil-i18n";
import type { WorldOptions } from "@tsukilabs/nil-bindings";

defineProps<{
  disabled: boolean;
}>();

const worldOptions = defineModel<Partial<WorldOptions>>({ required: true });

const sliderValue = computed({
  get: () => [worldOptions.value.botAdvancedStartRatio ?? CONSTS.botAdvancedStartRatioDefault],
  set: (value) => {
    worldOptions.value.botAdvancedStartRatio = value.at(0) ?? CONSTS.botAdvancedStartRatioDefault;
  },
});

const { t } = useI18n();
</script>

<template>
  <Label for="slider-bot-advanced-start-ratio">
    <span>{{ t("host-game.advanced-bots-ratio") }}</span>
    <div>
      <Slider
        id="slider-bot-advanced-start-ratio"
        v-model:model-value="sliderValue"
        :disabled
        :min="CONSTS.botAdvancedStartRatioMin"
        :max="CONSTS.botAdvancedStartRatioMax"
        :step="0.01"
      />
      <span>{{ formatPercent(sliderValue[0]) }}</span>
    </div>
  </Label>
</template>
