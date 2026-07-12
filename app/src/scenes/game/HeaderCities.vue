<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { computed, nextTick } from "vue";
import { cn } from "@/components/ui/utils";
import type { CityImpl } from "@/core/model/city/city";
import { CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList } from "@ui/command";

const props = defineProps<{
  currentCity: CityImpl;
  playerCities: readonly CityImpl[];
}>();

const open = defineModel<boolean>("open", { required: true });

const { t } = useI18n();

const items = computed(() => {
  return props.playerCities.map((city) => {
    return {
      value: city.coord.id,
      label: city.formatNameWithCoord(),
    };
  });
});

async function onClick(coordId: string) {
  await nextTick();
  const city = props.playerCities.find((it) => {
    return it.coord.id === coordId;
  });

  if (city && !city.coord.is(props.currentCity.coord)) {
    NIL.city.setCoord(city.coord).err();
  }

  open.value = false;
}
</script>

<template>
  <CommandDialog v-model:open="open">
    <CommandInput :placeholder="t('search')" />
    <CommandList>
      <CommandEmpty>{{ t("no-results-found") }}</CommandEmpty>
      <CommandGroup>
        <CommandItem v-for="item of items" :key="item.value" :value="item.value">
          <span
            :class="cn('w-full cursor-pointer', (item.value === currentCity.coord.id) && 'font-bold')"
            @click="() => void onClick(item.value)"
          >
            {{ item.label }}
          </span>
        </CommandItem>
      </CommandGroup>
    </CommandList>
  </CommandDialog>
</template>
