<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import { Label } from "@ui/label";
import { Checkbox } from "@ui/checkbox";
import { useI18n } from "@tsukilabs/nil-i18n";
import { useSettings } from "@/stores/settings";

const { t } = useI18n();

const settings = useSettings();
</script>

<template>
  <div class="size-full px-4">
    <div class="flex flex-col gap-4">
      <Label>
        <Checkbox v-model="settings.stable.hideUnmet" />
        <span>{{ t("infrastructure.hide-unavailable-units") }}</span>
      </Label>
    </div>
  </div>
</template>
