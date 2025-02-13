<script setup lang="ts">
import { computed } from 'vue';
import type { Option } from '@tb-dev/utils';
import {
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
} from '@/components/base/number-field';

type Props = {
  defaultValue?: number;
  disabled?: boolean;
  formatOptions?: Intl.NumberFormatOptions;
  max?: number;
  min?: number;
  modelValue: Option<number>;
};

const props = withDefaults(defineProps<Props>(), {
  formatOptions: () => ({ useGrouping: false }),
});

const emit = defineEmits<{
  (e: 'update:modelValue', value: null | number): void;
}>();

const value = computed<number | undefined>({
  // eslint-disable-next-line no-undefined
  get: () => props.modelValue ?? undefined,
  set: (it) => emit('update:modelValue', it ?? null),
});
</script>

<template>
  <NumberField v-model="value" :default-value :format-options :min :max :disabled>
    <NumberFieldContent>
      <NumberFieldDecrement />
      <NumberFieldInput />
      <NumberFieldIncrement />
    </NumberFieldContent>
  </NumberField>
</template>
