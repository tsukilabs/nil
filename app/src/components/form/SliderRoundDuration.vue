<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import { computed } from "vue";
import { Label } from "@ui/label";
import { Slider } from "@ui/slider";
import { Switch } from "@ui/switch";
import { CONSTS } from "@/lib/global";
import { useI18n } from "@tsukilabs/nil-i18n";
import type { RoundDuration } from "@tsukilabs/nil-bindings";

defineProps<{
  disabled: boolean;
}>();

const duration = defineModel<RoundDuration>("duration", { required: true });
const enabled = defineModel<boolean>("enabled", { required: true });

const sliderValue = computed({
  get: () => [duration.value],
  set: (value) => {
    duration.value = value.at(0) ?? CONSTS.roundDurationDefault;
  },
});

const { t } = useI18n();
</script>

<template>
  <Label for="slider-round-duration">
    <span>{{ t("host-game.round-duration") }}</span>
    <div class="flex flex-row items-center gap-1">
      <Switch v-model="enabled" />
      <Slider
        id="slider-round-duration"
        v-model:model-value="sliderValue"
        :disabled="disabled || !enabled"
        :min="CONSTS.roundDurationMin"
        :max="CONSTS.roundDurationMax"
        :step="1"
      />
      <span>{{ duration }}</span>
    </div>
  </Label>
</template>
