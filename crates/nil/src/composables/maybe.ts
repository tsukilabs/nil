import { type MaybeRefOrGetter, toValue } from 'vue';
import { isNullish, type Option } from '@tb-dev/utils';

export function maybe<T, U>(value: MaybeRefOrGetter<Option<T>>, fn: (value: T) => U): null | U {
  const _value = toValue(value);
  return isNullish(_value) ? null : fn(_value);
}
