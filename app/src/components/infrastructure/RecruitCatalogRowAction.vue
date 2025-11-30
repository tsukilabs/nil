<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import {
  Button,
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
} from '@tb-dev/vue-components';

defineProps<{
  canRecruit: boolean;
  loading: boolean;
  onRecruitOrder: (chunks: number) => void;
}>();

const chunks = defineModel<number>({ required: true });

const { t } = useI18n();

const { sm } = useBreakpoints();
</script>

<template>
  <div class="grid max-w-fit grid-cols-2 items-center justify-start gap-4">
    <NumberField
      v-model="chunks"
      :disabled="loading"
      :min="0"
      :max="9_999"
      :step="1"
      class="w-full"
    >
      <NumberFieldContent>
        <NumberFieldDecrement class="[&>svg]:max-sm:size-3" />
        <NumberFieldInput class="dark:bg-input/40 max-sm:h-6 max-sm:max-w-28 max-sm:text-xs" />
        <NumberFieldIncrement class="[&>svg]:max-sm:size-3" />
      </NumberFieldContent>
    </NumberField>

    <Button
      variant="default"
      :size="sm ? 'sm' : 'xs'"
      :disabled="!canRecruit || chunks < 1"
      class="max-w-24"
      @click="() => onRecruitOrder(Math.trunc(chunks))"
    >
      <span>{{ t('recruit') }}</span>
    </Button>
  </div>
</template>
