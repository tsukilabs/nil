<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { Label, NumberField, NumberFieldContent, NumberFieldInput } from '@tb-dev/vue-components';

defineProps<{
  destinationCity: Option<PublicCity>;
}>();

const destination = defineModel<Coord>({ required: true });

const { continentSize } = NIL.world.refs();
</script>

<template>
  <div class="w-max flex flex-col gap-2 border border-border rounded-md p-4">
    <div v-if="destinationCity" class="flex flex-col gap-1">
      <span class="text-sm text-muted-foreground">
        {{ `${destinationCity.name} (${destinationCity.owner.id})` }}
      </span>
    </div>

    <div class="flex items-center gap-4">
      <Label class="flex-row! items-center! gap-2!">
        <span class="w-max! text-xs md:text-sm text-muted-foreground">X</span>
        <NumberField
          v-model="destination.x"
          :min="0"
          :max="continentSize"
          :step="1"
          :default-value="0"
          class="w-full"
        >
          <NumberFieldContent>
            <NumberFieldInput class="dark:bg-input/40 max-sm:h-6 max-sm:max-w-28 max-sm:text-xs" />
          </NumberFieldContent>
        </NumberField>
      </Label>
      <Label class="flex-row! items-center! gap-2!">
        <span class="w-max! text-xs md:text-sm text-muted-foreground">Y</span>
        <NumberField
          v-model="destination.y"
          :min="0"
          :max="continentSize"
          :step="1"
          :default-value="0"
          class="w-full"
        >
          <NumberFieldContent>
            <NumberFieldInput class="dark:bg-input/40 max-sm:h-6 max-sm:max-w-28 max-sm:text-xs" />
          </NumberFieldContent>
        </NumberField>
      </Label>
    </div>
  </div>
</template>
