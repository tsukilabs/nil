<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import { computed } from "vue";
import { Label } from "@ui/label";
import { Slider } from "@ui/slider";
import { CONSTS } from "@/lib/global";
import { useI18n } from "@tsukilabs/nil-i18n";
import type { WorldOptions } from "@tsukilabs/nil-bindings";

defineProps<{
  disabled: boolean;
}>();

const worldOptions = defineModel<Partial<WorldOptions>>({ required: true });

const sliderValue = computed({
  get: () => [worldOptions.value.size ?? CONSTS.continentSizeDefault],
  set: (value) => {
    worldOptions.value.size = value.at(0) ?? CONSTS.continentSizeDefault;
  },
});

const { t } = useI18n();
</script>

<template>
  <Label for="slider-world-size">
    <span>{{ t("host-game.world-size") }}</span>
    <div>
      <Slider
        id="slider-world-size"
        v-model:model-value="sliderValue"
        :disabled
        :min="CONSTS.continentSizeMin"
        :max="CONSTS.continentSizeMax"
        :step="10"
      />
      <span>{{ sliderValue[0] }}</span>
    </div>
  </Label>
</template>
