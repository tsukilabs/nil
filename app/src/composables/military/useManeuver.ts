// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from "@/commands";
import { computed, ref, toRef, watch } from "vue";
import { ArmyImpl } from "@/core/model/military/army";
import type { ManeuverId } from "@tsukilabs/nil-bindings";
import { ManeuverImpl } from "@/core/model/military/maneuver";
import { asyncComputed, asyncRef, type MaybeNilRef, useMutex } from "@tb-dev/vue";

export function useManeuver(id: MaybeNilRef<ManeuverId>) {
  const idRef = toRef(id);
  const {
    state: maneuver,
    loading: isLoadingManeuver,
    load: loadManeuver,
  } = asyncRef(null, async () => {
    return idRef.value ? ManeuverImpl.load(idRef.value) : null;
  });

  const { round } = NIL.round.refs();
  const { id: player } = NIL.player.refs();
  const { locked, lock } = useMutex();

  const armyId = computed(() => maneuver.value?.army);
  const isLoadingArmyOwner = ref(false);
  const armyOwner = asyncComputed(null, async () => {
    if (armyId.value) {
      return commands.getArmyOwner(armyId.value);
    }
    else {
      return null;
    }
  }, {
    evaluating: isLoadingArmyOwner,
  });

  const isArmyOwnedByCurrentPlayer = computed(() => {
    return (
      armyOwner.value?.kind === "player" &&
      armyOwner.value.id === player.value
    );
  });

  const isLoadingArmy = ref(false);
  const army = asyncComputed(null, async () => {
    if (
      player.value &&
      armyId.value &&
      isArmyOwnedByCurrentPlayer.value
    ) {
      return ArmyImpl.load(armyId.value);
    }
    else {
      return null;
    }
  }, {
    evaluating: isLoadingArmy,
  });

  const isLoadingCities = ref(false);
  const cities = asyncComputed(null, async () => {
    return maneuver.value?.getCities();
  }, {
    evaluating: isLoadingCities,
  });

  const loading = computed(() => {
    return (
      locked.value ||
      isLoadingArmy.value ||
      isLoadingArmyOwner.value ||
      isLoadingCities.value ||
      isLoadingManeuver.value
    );
  });

  watch(() => round.value?.id, loadManeuver);

  async function cancelManeuver() {
    await lock(async () => {
      if (maneuver.value && isArmyOwnedByCurrentPlayer.value) {
        await commands.cancelManeuver(maneuver.value.id);
        await loadManeuver();
      }
    });
  }

  return {
    army,
    armyOwner,
    cities,
    isArmyOwnedByCurrentPlayer,
    maneuver: maneuver as Readonly<typeof maneuver>,
    loading,
    cancelManeuver,
    loadManeuver,
  };
}
