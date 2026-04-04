import { twMerge } from 'tailwind-merge';
import { type ClassValue, clsx } from 'clsx';
import type { CheckboxRootProps } from 'reka-ui';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function toBooleanCheckboxValue(value: Option<CheckboxRootProps['modelValue']>): boolean {
  return value === true;
}
