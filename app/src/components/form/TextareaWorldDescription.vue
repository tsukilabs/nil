<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import { Label } from "@ui/label";
import { Textarea } from "@ui/textarea";
import type { Option } from "@tb-dev/utils";
import { useI18n } from "@tsukilabs/nil-i18n";

defineProps<{
  disabled: boolean;
}>();

const description = defineModel<Option<string>>({ required: true });

const { t } = useI18n();
</script>

<template>
  <Label for="textarea-world-description">
    <span>{{ t("host-game.world-description") }}</span>
    <Textarea
      id="textarea-world-description"
      v-model="description"
      type="text"
      spellcheck="false"
      autocapitalize="off"
      autocomplete="off"
      :disabled
      :maxlength="300"
      class="max-h-24 resize-none"
    />
  </Label>
</template>
