<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { computed, ref } from 'vue';
import * as commands from '@/commands';
import SquadGrid from './SquadGrid.vue';
import Maneuvers from './Maneuvers.vue';
import { handleError } from '@/lib/error';
import { asyncComputed } from '@tb-dev/vue';
import Destination from './Destination.vue';
import { Button } from '@tb-dev/vue-components';
import { CoordImpl } from '@/core/model/continent/coord';
import { useManeuvers } from '@/composables/military/useManeuvers';
import { ArmyPersonnelImpl } from '@/core/model/military/army-personnel';
import { foldArmyPersonnel } from '@/composables/military/foldArmyPersonnel';
import { useOwnIdleArmiesAt } from '@/composables/military/useOwnIdleArmiesAt';

const { t } = useI18n();

const origin = ref(getDefaultCoord());
const destination = ref(CoordImpl.splat(0));
const destinationCity = asyncComputed(null, () => {
  return commands.findPublicCity(destination.value);
});

const armies = useOwnIdleArmiesAt(origin);
const available = foldArmyPersonnel(armies);
const personnel = ref(ArmyPersonnelImpl.createEmpty());

const maneuvers = useManeuvers(origin);

const canSend = computed(() => {
  return (
    !personnel.value.isEmpty() &&
    !origin.value.is(destination.value) &&
    Boolean(destinationCity.value)
  );
});

await NIL.military.update();

async function send(kind: ManeuverKind) {
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

function clear() {
  personnel.value = ArmyPersonnelImpl.createEmpty();
}

function getDefaultCoord() {
  return NIL.city.getCoord() ??
    NIL.player.getCoords().at(0) ??
    CoordImpl.splat(0);
}
</script>

<template>
  <div class="size-full flex flex-col gap-8 px-4">
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

    <Maneuvers v-if="maneuvers.length > 0" :maneuvers />
  </div>
</template>
