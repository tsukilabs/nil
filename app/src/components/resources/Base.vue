<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { formatInt } from '@/lib/intl';
import { useBreakpoints } from '@/composables/util/useBreakpoints';

const props = defineProps<{
  amount?: Option<number>;
  limit?: Option<number>;
  color: string;
  alwaysLiteral?: boolean;
}>();

const { md } = useBreakpoints();

const fractionIntl = new Intl.NumberFormat(undefined, {
  style: 'decimal',
  maximumFractionDigits: 1,
  minimumFractionDigits: 0,
  roundingMode: 'trunc',
  trailingZeroDisplay: 'stripIfInteger',
  notation: 'standard',
  useGrouping: 'auto',
  localeMatcher: 'best fit',
});

function format() {
  const value = props.amount ?? 0;
  if (!props.alwaysLiteral && !md.value) {
    if (value >= 1_000 && value <= 99_999) {
      return `${fractionIntl.format(value / 1_000)}k`;
    }
    else if (value >= 100_000 && value <= 999_999) {
      return `${formatInt(value / 1_000)}k`;
    }
    else if (value >= 1_000_000 && value <= 9_999_999) {
      return `${fractionIntl.format(value / 1_000_000)}M`;
    }
    else if (value >= 10_000_000 && value <= 999_999_999) {
      return `${formatInt(value / 1_000_000)}M`;
    }
    else if (value >= 1_000_000_000) {
      return `${fractionIntl.format(value / 1_000_000_000)}B`;
    }
  }

  return formatInt(value);
}

function isOverflowing() {
  return (
    typeof props.amount === 'number' &&
    typeof props.limit === 'number' &&
    props.amount >= props.limit
  );
}
</script>

<template>
  <div class="flex min-w-max items-center justify-start gap-1">
    <div
      class="size-3 min-h-3 min-w-3 overflow-hidden rounded-full"
      :style="{ backgroundColor: color }"
    >
    </div>

    <div :class="isOverflowing() ? 'text-red-400' : null">
      {{ format() }}
    </div>
  </div>
</template>
