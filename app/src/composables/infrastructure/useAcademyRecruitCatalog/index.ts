// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { shallowRef } from 'vue';
import { useMutex } from '@tb-dev/vue';
import { toCoordRef } from '@/composables/util/toRef';
import type { CoordImpl } from '@/core/model/continent/coord';
import {
  addAcademyRecruitOrder,
  cancelAcademyRecruitOrder,
  getAcademyRecruitCatalog,
} from '@/commands';

export function useAcademyRecruitCatalog(coord?: MaybeNilRef<CoordImpl>) {
  const coordRef = toCoordRef(coord);
  const catalog = shallowRef<Option<AcademyRecruitCatalog>>();
  const { locked, lock } = useMutex();

  async function load() {
    await lock(async () => {
      if (coordRef.value) {
        catalog.value = await getAcademyRecruitCatalog(coordRef.value);
      }
    });
  }

  async function add(unit: AcademyUnitId, chunks: number) {
    await lock(async () => {
      if (coordRef.value && chunks > 0) {
        await addAcademyRecruitOrder({ coord: coordRef.value, unit, chunks });
      }
    });

    await load();
  }

  async function cancel(id: InfrastructureQueueOrderId) {
    await lock(async () => {
      if (coordRef.value) {
        await cancelAcademyRecruitOrder(coordRef.value, id);
      }
    });

    await load();
  }

  return {
    catalog: catalog as Readonly<typeof catalog>,
    loading: locked,
    load,
    add,
    cancel,
  };
}
