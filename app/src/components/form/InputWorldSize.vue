<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { Label } from "@ui/label";
import { useI18n } from "vue-i18n";
import { CONSTS } from "@/lib/global";
import type { Option } from "@tb-dev/utils";
import enUS from "@/locale/en-US/scenes/host-game.json";
import ptBR from "@/locale/pt-BR/scenes/host-game.json";
import {
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
} from "@ui/number-field";

defineProps<{
  disabled: boolean;
}>();

const size = defineModel<Option<number>>({ required: true });

const { t } = useI18n({
  messages: {
    "en-US": enUS,
    "pt-BR": ptBR,
  },
});
</script>

<template>
  <Label>
    <span>{{ t("world-size") }}</span>
    <NumberField
      v-model="size"
      :disabled
      :min="CONSTS.continentSizeMin"
      :max="CONSTS.continentSizeMax"
      :step="10"
      class="w-full"
    >
      <NumberFieldContent>
        <NumberFieldDecrement />
        <NumberFieldInput class="dark:bg-input/40" />
        <NumberFieldIncrement />
      </NumberFieldContent>
    </NumberField>
  </Label>
</template>
