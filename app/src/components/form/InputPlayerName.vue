<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import { Input } from "@ui/input";
import { Label } from "@ui/label";
import type { Option } from "@tb-dev/utils";
import { useI18n } from "@tsukilabs/nil-i18n";
import type { PlayerId } from "@tsukilabs/nil-bindings";

defineProps<{
  disabled?: boolean;
}>();

const name = defineModel<Option<PlayerId>>({ required: true });

const { t } = useI18n();
</script>

<template>
  <Label for="input-player-name">
    <span>{{ t("player-name") }}</span>
    <Input
      id="input-player-name"
      v-model="name"
      type="text"
      :disabled
      :minlength="1"
      :maxlength="20"
      spellcheck="false"
      autocapitalize="off"
      autocomplete="off"
    />
  </Label>
</template>
