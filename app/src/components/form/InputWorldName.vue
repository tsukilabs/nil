<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import { Input } from "@ui/input";
import { Label } from "@ui/label";
import { useI18n } from "@tsukilabs/nil-i18n";
import enUS from "@/locale/en-US/scenes/host-game.json";
import ptBR from "@/locale/pt-BR/scenes/host-game.json";
import type { WorldOptions } from "@tsukilabs/nil-bindings";

defineProps<{
  disabled: boolean;
}>();

const worldOptions = defineModel<Partial<WorldOptions>>({ required: true });

const { t } = useI18n({
  messages: {
    "en-US": enUS,
    "pt-BR": ptBR,
  },
});
</script>

<template>
  <Label for="input-world-name">
    <span>{{ t("world-name") }}</span>
    <Input
      id="input-world-name"
      v-model="worldOptions.name"
      type="text"
      :disabled
      :minlength="1"
      :maxlength="30"
      spellcheck="false"
      autocapitalize="off"
      autocomplete="off"
    />
  </Label>
</template>
