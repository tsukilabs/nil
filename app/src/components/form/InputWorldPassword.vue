<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import { Input } from "@ui/input";
import { Label } from "@ui/label";
import type { Option } from "@tb-dev/utils";
import { useI18n } from "@tsukilabs/nil-i18n";

defineProps<{
  disabled: boolean;
  onEnter?: () => void;
}>();

const password = defineModel<Option<string>>({ required: true });

const { t } = useI18n();
</script>

<template>
  <Label for="input-world-password">
    <span>{{ t("online.world-password") }}</span>
    <Input
      id="input-world-password"
      v-model="password"
      type="password"
      :disabled
      :minlength="3"
      :maxlength="50"
      spellcheck="false"
      autocapitalize="off"
      autocomplete="off"
      @keydown.enter="() => onEnter?.()"
    />
  </Label>
</template>
