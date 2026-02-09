// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed, type Ref, toRef } from 'vue';
import type { WritablePartial } from '@tb-dev/utils';
import { WorldConfigImpl } from '@/core/model/world-config';

export function useBotAdvancedStartRatioSlider(
  worldOptions: Ref<WritablePartial<WorldOptions>>,
) {
  const optionsRef = toRef(worldOptions);
  return computed({
    get: () => [
      optionsRef.value.botAdvancedStartRatio ?? WorldConfigImpl.DEFAULT_BOT_ADVANCED_START_RATIO,
    ],
    set: (value) => {
      optionsRef.value.botAdvancedStartRatio = value.at(0) ??
        WorldConfigImpl.DEFAULT_BOT_ADVANCED_START_RATIO;
    },
  });
}
