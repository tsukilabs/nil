// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from "@/commands";
import { computed, ref, toRef, watch } from "vue";
import { ArmyImpl } from "@/core/model/military/army";
import type { ManeuverId } from "@tsukilabs/nil-bindings";
import { ManeuverImpl } from "@/core/model/military/maneuver";
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

  const isLoadingArmy = ref(false);
  const army = asyncComputed(null, async () => {
    if (
      player.value &&
      armyId.value &&
      armyOwner.value?.kind === "player" &&
      armyOwner.value.id === player.value
    ) {
      return ArmyImpl.load(armyId.value);
    }
    else {
      return null;
    }
  }, {
    evaluating: isLoadingArmy,
  });

  const loading = computed(() => {
    return (
      isLoadingArmy.value ||
      isLoadingArmyOwner.value ||
      isLoadingManeuver.value
    );
  });

  return {
    army,
    armyOwner,
    maneuver: maneuver as Readonly<typeof maneuver>,
    loading,
    loadManeuver,
  };
}
