<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed, watch } from "vue";
import { onKeyDown } from "@tb-dev/vue";
import type { Option } from "@tb-dev/utils";
import { throttle } from "es-toolkit/function";
import { ListenerSet } from "@/lib/listener-set";
import type { CoordImpl } from "@/core/model/continent/coord";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@ui/select";
import { useIdleArmiesPublicCities } from "@/composables/military/useIdleArmiesPublicCities";

const coord = defineModel<Option<CoordImpl>>({ required: true });

const {
  cities,
  key,
  loading,
  loadCoords,
} = useIdleArmiesPublicCities();

const coordId = computed({
  get: () => coord.value?.id ?? null,
  set: (id) => {
    const city = cities.value.find((it) => {
      return it.coord.id === id;
    });

    coord.value = city?.coord ?? null;
  },
});

const { round } = NIL.round.refs();

const listeners = new ListenerSet();
listeners.event.onMilitary(loadCoords);

watch(() => round.value?.id, loadCoords);

if (__DESKTOP__) {
  onKeyDown("F5", throttle(loadCoords, 1000));
}
</script>

<template>
  <div>
    <Select
      :key
      v-model="coordId"
      :disabled="loading || cities.length === 0"
    >
      <SelectTrigger class="w-full">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem
          v-for="city of cities"
          :key="city.coord.id"
          :value="city.coord.id"
        >
          <span>{{ city.formatNameWithCoord() }}</span>
        </SelectItem>
      </SelectContent>
    </Select>
  </div>
</template>
