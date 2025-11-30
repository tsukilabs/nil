// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { shallowRef } from 'vue';
import { useMutex } from '@tb-dev/vue';
import { toCoordRef } from '@/composables/util/toRef';
import type { CoordImpl } from '@/core/model/continent/coord';
import {
  addPrefectureBuildOrder,
  cancelPrefectureBuildOrder,
  getPrefectureBuildCatalog,
  toggleBuilding,
} from '@/commands';

export function usePrefectureBuildCatalog(coord?: MaybeNilRef<CoordImpl>) {
  const coordRef = toCoordRef(coord);
  const catalog = shallowRef<Option<PrefectureBuildCatalog>>();
  const { locked, lock } = useMutex();

  async function load() {
    await lock(async () => {
      if (coordRef.value) {
        catalog.value = await getPrefectureBuildCatalog(coordRef.value);
      }
    });
  }

  async function add(building: BuildingId, kind: PrefectureBuildOrderKind) {
    await lock(async () => {
      if (coordRef.value) {
        await addPrefectureBuildOrder({ coord: coordRef.value, building, kind });
      }
    });

    await load();
  }

  async function cancel() {
    await lock(async () => {
      if (coordRef.value) {
        await cancelPrefectureBuildOrder(coordRef.value);
      }
    });

    await load();
  }

  async function toggle(id: BuildingId, enabled: boolean) {
    await lock(async () => {
      if (coordRef.value) {
        await toggleBuilding(coordRef.value, id, enabled);
      }
    });
  }

  return {
    catalog: catalog as Readonly<typeof catalog>,
    loading: locked,
    load,
    add,
    cancel,
    toggle,
  };
}
