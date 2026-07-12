<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from "vue";
import { Label } from "@ui/label";
import { useI18n } from "vue-i18n";
import { Slider } from "@ui/slider";
import { CONSTS } from "@/lib/global";
import { formatPercent } from "@/lib/intl";
import enUS from "@/locale/en-US/scenes/host-game.json";
import ptBR from "@/locale/pt-BR/scenes/host-game.json";
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

const { t } = useI18n({
  messages: {
    "en-US": enUS,
    "pt-BR": ptBR,
  },
});
</script>

<template>
  <Label>
    <span>{{ t("advanced-bots-ratio") }}</span>
    <div>
      <Slider
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
