<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import * as commands from '@/commands';
import Maneuvers from './Maneuvers.vue';
import SquadGrid from './SquadGrid.vue';
import { handleError } from '@/lib/error';
import Destination from './Destination.vue';
import { asyncComputed } from '@tb-dev/vue';
import { computed, nextTick, ref } from 'vue';
import { Button } from '@tb-dev/vue-components';
import { useManeuvers } from '@/composables/military/useManeuvers';
import { usePlayerTurn } from '@/composables/player/usePlayerTurn';
import { ArmyPersonnelImpl } from '@/core/model/military/army-personnel';
import { useWarRoomCoords } from '@/composables/military/useWarRoomCoords';
import { foldArmyPersonnel } from '@/composables/military/foldArmyPersonnel';
import { useOwnIdleArmiesAt } from '@/composables/military/useOwnIdleArmiesAt';

const { t } = useI18n();

const isPlayerTurn = usePlayerTurn();

const { origin, destination } = useWarRoomCoords();
const destinationCity = asyncComputed(null, async () => {
  const search = { coord: [destination.value] };
  const city = await commands.searchPublicCity(search);
  return city.at(0) ?? null;
});

const armies = useOwnIdleArmiesAt(origin);
const available = foldArmyPersonnel(armies);
const personnel = ref(ArmyPersonnelImpl.createEmpty());

const maneuvers = useManeuvers(origin);

const canSend = computed(() => {
  return (
    isPlayerTurn.value &&
    !personnel.value.isEmpty() &&
    !origin.value.is(destination.value) &&
    Boolean(destinationCity.value)
  );
});

await NIL.military.update();

async function send(kind: ManeuverKind) {
  await nextTick();
  if (canSend.value) {
    try {
      await commands.requestManeuver({
        kind,
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

function clear() {
  personnel.value = ArmyPersonnelImpl.createEmpty();
}
</script>

<template>
  <div class="w-full flex flex-col gap-4 px-4 xl:flex-row xl:gap-8">
    <div class="w-full flex flex-col gap-8">
      <SquadGrid v-model="personnel" :available />

      <Destination v-model="destination" :destination-city />

      <div class="grid grid-cols-3 items-center justify-start gap-4 max-w-max">
        <Button variant="default" :disabled="!canSend" @click="() => send('attack')">
          <span>{{ t('attack') }}</span>
        </Button>
        <Button variant="default" :disabled="!canSend" @click="() => send('support')">
          <span>{{ t('support') }}</span>
        </Button>
        <Button variant="secondary" @click="clear">
          <span>{{ t('clear') }}</span>
        </Button>
      </div>
    </div>

    <Maneuvers v-if="maneuvers.length > 0" :maneuvers />
  </div>
</template>
