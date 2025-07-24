// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { handleError } from '@/lib/error';
import type { MaybeNilRef } from '@tb-dev/vue';
import { readonly, ref, shallowRef } from 'vue';
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
  const loading = ref(false);

  async function load() {
    if (coordRef.value && !loading.value) {
      try {
        loading.value = true;
        catalog.value = await getPrefectureBuildCatalog(coordRef.value);
      } catch (err) {
        handleError(err);
      } finally {
        loading.value = false;
      }
    }
  }

  async function add(building: BuildingId, kind: PrefectureBuildOrderKind) {
    if (coordRef.value && !loading.value) {
      try {
        loading.value = true;
        await addPrefectureBuildOrder({ coord: coordRef.value, building, kind });
        await load();
      } catch (err) {
        handleError(err);
      } finally {
        loading.value = false;
      }
    }
  }

  async function cancel() {
    if (coordRef.value && !loading.value) {
      try {
        loading.value = true;
        await cancelPrefectureBuildOrder(coordRef.value);
        await load();
      } catch (err) {
        handleError(err);
      } finally {
        loading.value = false;
      }
    }
  }

  async function toggle(id: BuildingId, enabled: boolean) {
    if (coordRef.value && !loading.value) {
      try {
        loading.value = true;
        await toggleBuilding(coordRef.value, id, enabled);
      } catch (err) {
        handleError(err);
      } finally {
        loading.value = false;
      }
    }
  }

  return {
    catalog: catalog as Readonly<typeof catalog>,
    loading: readonly(loading),
    load,
    add,
    cancel,
    toggle,
  };
}
