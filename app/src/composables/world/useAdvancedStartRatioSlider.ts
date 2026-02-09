// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WritablePartial } from '@tb-dev/utils';
import { computed, type MaybeRefOrGetter, toRef } from 'vue';

export function useAdvancedStartRatioSlider(
  worldOptions: MaybeRefOrGetter<WritablePartial<WorldOptions>>,
) {
  const optionsRef = toRef(worldOptions);
  return computed({
    get: () => [optionsRef.value.advancedStartRatio ?? 0.2],
    set: (value) => {
      optionsRef.value.advancedStartRatio = value.at(0) ?? 0.2;
    },
  });
}
