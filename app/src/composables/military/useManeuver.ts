// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from "@/commands";
import { computed, ref, toRef, watch } from "vue";
import { ArmyImpl } from "@/core/model/military/army";
import type { ManeuverId } from "@tsukilabs/nil-bindings";
import { ManeuverImpl } from "@/core/model/military/maneuver";
import { PublicCityImpl } from "@/core/model/city/public-city";
import { asyncComputed, asyncRef, type MaybeNilRef } from "@tb-dev/vue";

export function useManeuver(id: MaybeNilRef<ManeuverId>) {
  const { round } = NIL.round.refs();
  const { id: player } = NIL.player.refs();

  const idRef = toRef(id);
  const {
    state: maneuver,
    loading: isLoadingManeuver,
    load: loadManeuver,
  } = asyncRef(null, async () => {
    return idRef.value ? ManeuverImpl.load(idRef.value) : null;
  });

  watch(() => round.value?.id, loadManeuver);

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
    if (maneuver.value) {
      const origin = maneuver.value.origin;
      const destination = maneuver.value.destination;
      const impl = await PublicCityImpl.bulkLoad([origin, destination]);
      return {
        origin: impl.find((city) => city.coord.is(origin)) ?? null,
        destination: impl.find((city) => city.coord.is(destination)) ?? null,
      } as const;
    }
    else {
      return null;
    }
  }, {
    evaluating: isLoadingCities,
  });

  const loading = computed(() => {
    return (
      isLoadingArmy.value ||
      isLoadingArmyOwner.value ||
      isLoadingCities.value ||
      isLoadingManeuver.value
    );
  });

  return {
    army,
    armyOwner,
    cities,
    isArmyOwnedByCurrentPlayer,
    maneuver: maneuver as Readonly<typeof maneuver>,
    loading,
    loadManeuver,
  };
}
