// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ResourcesImpl } from '@/core/model/resources';
import { computed, type MaybeRefOrGetter, ref, toRef } from 'vue';

type RecruitCatalogEntry = AcademyRecruitCatalogEntry | StableRecruitCatalogEntry;

export function useRecruitCatalogEntry(entry: MaybeRefOrGetter<RecruitCatalogEntry>) {
  const entryRef = toRef(entry);

  const chunks = ref(0);
  const minChunk = computed(() => {
    return Math.max(Math.trunc(chunks.value), 1);
  });

  const resources = computed(() => {
    if (entryRef.value.kind === 'available') {
      const impl = ResourcesImpl.create(entryRef.value.recipe.resources);
      return impl.mul(minChunk.value);
    }

    return ResourcesImpl.zero();
  });

  const maintenance = computed(() => {
    if (entryRef.value.kind === 'available') {
      return entryRef.value.recipe.maintenance * minChunk.value;
    }

    return 0;
  });

  const workforce = computed(() => {
    if (entryRef.value.kind === 'available') {
      return entryRef.value.recipe.workforce * minChunk.value;
    }

    return 0;
  });

  return {
    chunks,
    resources,
    maintenance,
    workforce,
  };
}
