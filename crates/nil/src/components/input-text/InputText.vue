<script setup lang="ts">
import { computed } from 'vue';
import { cn } from '@/lib/utils';
import type { Option } from '@tb-dev/utils';
import { Input as BaseInput } from '@/components/base/input';

type Props = {
  class?: string;
  disabled?: boolean;
  modelValue: Option<string>;
  onBlur?: () => void;
  placeholder?: string;
};

const props = defineProps<Props>();

const emit = defineEmits<{
  'update:modelValue': [value: null | string];
}>();

const value = computed<string | undefined>({
  // eslint-disable-next-line no-undefined
  get: () => props.modelValue ?? undefined,
  set: (it) => emit('update:modelValue', it ?? null),
});
</script>

<template>
  <BaseInput
    v-model.trim="value"
    type="text"
    :placeholder
    :disabled
    :class="cn('focus-visible:ring-0 disabled:cursor-default', props.class)"
    @blur="onBlur"
  />
</template>
