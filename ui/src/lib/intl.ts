// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export const integerIntl = new Intl.NumberFormat(undefined, {
  style: 'decimal',
  maximumFractionDigits: 0,
  roundingMode: 'trunc',
  notation: 'standard',
  useGrouping: 'auto',
  localeMatcher: 'best fit',
});

export function formatInt(value: number) {
  return integerIntl.format(value);
}

export const sortCollator = new Intl.Collator(undefined, {
  numeric: true,
  sensitivity: 'variant',
  usage: 'sort',
});

export function compare(a: string, b: string): number {
  return sortCollator.compare(a, b);
}
