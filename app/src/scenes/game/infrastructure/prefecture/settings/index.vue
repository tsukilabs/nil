<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import { ref } from "vue";
import { Input } from "@ui/input";
import { Label } from "@ui/label";
import { Button } from "@ui/button";
import { renameCity } from "@/commands";
import { Checkbox } from "@ui/checkbox";
import { useI18n } from "@tsukilabs/nil-i18n";
import { useSettings } from "@/stores/settings";

const { t } = useI18n();

const { coord, city } = NIL.city.refs();

const settings = useSettings();

const cityName = ref(city.value?.name);

function rename() {
  if (coord.value && cityName.value) {
    renameCity(coord.value, cityName.value).err();
  }
}
</script>

<template>
  <div class="size-full px-4">
    <div class="flex flex-col gap-4">
      <div class="flex flex-col gap-2">
        <Label>
          <Checkbox v-model="settings.prefecture.hideMaxed" />
          <span>{{ t("prefecture.hide-fully-constructed") }}</span>
        </Label>

        <Label>
          <Checkbox v-model="settings.prefecture.hideUnmet" />
          <span>{{ t("prefecture.hide-unavailable-buildings") }}</span>
        </Label>
      </div>

      <Label class="max-w-96 py-1">
        <span class="text-muted-foreground">{{ t("prefecture.rename-city") }}</span>
        <div class="flex items-center gap-2">
          <Input
            v-model.trim="cityName"
            type="text"
            minlength="1"
            maxlength="50"
            spellcheck="false"
          />
          <Button size="sm" :disabled="!city" @click="rename">
            <span>{{ t("rename") }}</span>
          </Button>
        </div>
      </Label>
    </div>
  </div>
</template>
