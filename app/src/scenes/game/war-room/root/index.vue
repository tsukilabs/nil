<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { Button } from "@ui/button";
import * as commands from "@/commands";
import { handleError } from "@/lib/error";
import { computed, nextTick, ref } from "vue";
import { throttle } from "es-toolkit/function";
import Cities from "@/scenes/game/war-room/root/Cities.vue";
import Maneuvers from "@/scenes/game/war-room/root/Maneuvers.vue";
import SquadGrid from "@/scenes/game/war-room/root/SquadGrid.vue";
import { usePlayerTurn } from "@/composables/player/usePlayerTurn";
import Destination from "@/scenes/game/war-room/root/Destination.vue";
import { useManeuversAt } from "@/composables/military/useManeuversAt";
import { asyncComputed, onKeyDown, useBreakpoints } from "@tb-dev/vue";
import type { ManeuverId, ManeuverKind } from "@tsukilabs/nil-bindings";
import { ArmyPersonnelImpl } from "@/core/model/military/army-personnel";
import { useWarRoomCoords } from "@/composables/military/useWarRoomCoords";
import { foldArmyPersonnel } from "@/composables/military/foldArmyPersonnel";
import { useOwnIdleArmiesAt } from "@/composables/military/useOwnIdleArmiesAt";

const { t } = useI18n();

const { player } = NIL.player.refs();
const isPlayerTurn = usePlayerTurn();

const { origin, destination } = useWarRoomCoords();
const destinationCity = asyncComputed(null, async () => {
  const search = { coord: [destination.value.toJSON()] };
  const city = await commands.searchPublicCity(search);
  return city.at(0) ?? null;
});

const armies = useOwnIdleArmiesAt(origin);
const availablePersonnel = foldArmyPersonnel(armies);
const personnel = ref(ArmyPersonnelImpl.createEmpty());

const maneuvers = useManeuversAt(origin);

const { sm } = useBreakpoints();

const canSend = computed(() => {
  return (
    player.value &&
    origin.value &&
    isPlayerTurn.value &&
    !personnel.value.isEmpty() &&
    !origin.value.is(destination.value) &&
    Boolean(destinationCity.value)
  );
});

if (__DESKTOP__) {
  onKeyDown("F5", throttle(NIL.military.update, 1000));
}

async function request(kind: ManeuverKind) {
  await nextTick();
  if (canSend.value && player.value && origin.value) {
    try {
      await commands.requestManeuver({
        kind,
        ruler: player.value.toRuler(),
        origin: origin.value,
        destination: destination.value,
        personnel: personnel.value.normalize(),
      });
    }
    catch (err) {
      handleError(err);
    }
  }
}

function cancel(id: ManeuverId) {
  commands.cancelManeuver(id).err();
}

function clear() {
  personnel.value = ArmyPersonnelImpl.createEmpty();
}

await NIL.military.update();
</script>

<template>
  <div class="w-full flex flex-col gap-4 px-4 xl:flex-row xl:gap-8">
    <div class="w-full flex flex-col gap-8 pt-1">
      <div class="w-full lg:min-w-max lg:max-w-1/2 grid grid-cols-1 gap-8">
        <Cities v-model="origin" />
        <SquadGrid v-model="personnel" :available="availablePersonnel" />
        <Destination v-model="destination" :destination-city />
      </div>

      <div class="max-sm:w-full sm:max-w-max grid grid-cols-3 items-center justify-center sm:justify-start gap-4">
        <Button
          variant="default"
          :size="sm ? 'default' : 'sm'"
          :disabled="!canSend"
          @click.stop="() => request('attack')"
        >
          <span>{{ t("attack") }}</span>
        </Button>
        <Button
          variant="default"
          :size="sm ? 'default' : 'sm'"
          :disabled="!canSend"
          @click.stop="() => request('support')"
        >
          <span>{{ t("support") }}</span>
        </Button>
        <Button
          variant="secondary"
          :size="sm ? 'default' : 'sm'"
          @click.stop="clear"
        >
          <span>{{ t("clear") }}</span>
        </Button>
      </div>
    </div>

    <Maneuvers
      v-if="origin && maneuvers.length > 0"
      :maneuvers
      :war-room-origin="origin"
      class="lg:max-w-1/2"
      @cancel-maneuver="cancel"
    />
  </div>
</template>
