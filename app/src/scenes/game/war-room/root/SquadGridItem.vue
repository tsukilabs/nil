<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { formatInt } from '@/lib/intl';
import { Label, NumberField, NumberFieldContent, NumberFieldInput } from '@tb-dev/vue-components';

defineProps<{
  available: Squad['size'];
}>();

const squad = defineModel<Squad>({ required: true });

const { t } = useI18n();
</script>

<template>
  <div>
    <Label>
      <span class="text-xs md:text-sm text-muted-foreground">
        {{ `${t(squad.unit)} (${formatInt(available)})` }}
      </span>
      <NumberField
        v-model="squad.size"
        :min="0"
        :max="available"
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
</template>
