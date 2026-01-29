// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { InjectionKey } from 'vue';
import { tryInjectOrElse } from '@tb-dev/vue';
import { breakpointsTailwind, useBreakpoints as original } from '@vueuse/core';

export type Breakpoints = ReturnType<typeof original<keyof typeof breakpointsTailwind>>;

const breakpointsKey = Symbol() as InjectionKey<Breakpoints>;

export function useBreakpoints(): Breakpoints {
  return tryInjectOrElse(breakpointsKey, () => {
    return original(breakpointsTailwind);
  });
}
