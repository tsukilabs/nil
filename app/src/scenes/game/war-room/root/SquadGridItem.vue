<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { formatInt } from '@/lib/intl';
import { Label, NumberField, NumberFieldContent, NumberFieldInput } from '@tb-dev/vue-components';

const props = defineProps<{
  available: Squad['size'];
}>();

const squad = defineModel<Writable<Squad>>({ required: true });

const { t } = useI18n();

function toggleMax() {
  if (squad.value.size !== props.available) {
    squad.value.size = props.available;
  }
  else {
    squad.value.size = 0;
  }
}
</script>

<template>
  <div>
    <Label>
      <div class="text-xs 2xl:text-sm text-muted-foreground">
        <span>{{ t(squad.unit) }}</span>
        <span class="cursor-pointer" @click="toggleMax">
          {{ ` (${formatInt(available)})` }}
        </span>
      </div>
      <NumberField
        v-model="squad.size"
        :min="0"
        :max="available"
        :step="1"
        :default-value="0"
        invert-wheel-change
        class="w-full"
      >
        <NumberFieldContent>
          <NumberFieldInput class="dark:bg-input/40 max-sm:h-6 max-sm:text-xs" />
        </NumberFieldContent>
      </NumberField>
    </Label>
  </div>
</template>
