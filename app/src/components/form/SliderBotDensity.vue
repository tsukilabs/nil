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
  get: () => [worldOptions.value.botDensity ?? CONSTS.botDensityDefault],
  set: (value) => {
    worldOptions.value.botDensity = value.at(0) ?? CONSTS.botDensityDefault;
  },
});

const { t } = useI18n();
</script>

<template>
  <Label for="slider-bot-density">
    <span>{{ t("host-game.bot-density") }}</span>
    <div>
      <Slider
        id="slider-bot-density"
        v-model:model-value="sliderValue"
        :disabled
        :min="CONSTS.botDensityMin"
        :max="CONSTS.botDensityMax"
        :step="0.01"
      />
      <span>{{ formatPercent(sliderValue[0]) }}</span>
    </div>
  </Label>
</template>
