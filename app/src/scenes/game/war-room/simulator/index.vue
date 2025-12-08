<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { computed, ref } from 'vue';
import { handleError } from '@/lib/error';
import { Button } from '@tb-dev/vue-components';
import PersonnelTable from './PersonnelTable.vue';
import BattleResultTable from './BattleResultTable.vue';
import enUS from '@/locale/en-US/scenes/game/war-room.json';
import ptBR from '@/locale/pt-BR/scenes/game/war-room.json';
import { BattleResultImpl } from '@/core/model/battle-result';
import { ArmyPersonnelImpl } from '@/core/model/military/army-personnel';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const { stats } = NIL.world.refs();

const result = ref<Option<BattleResultImpl>>();

const attacker = ref(ArmyPersonnelImpl.createEmpty());
const defender = ref(ArmyPersonnelImpl.createEmpty());
const wallLevel = ref(stats.value?.getBuildingMinLevel('wall'));

const canSimulate = computed(() => !attacker.value.isEmpty());

async function simulate() {
  try {
    result.value = await BattleResultImpl.simulate({
      attacker: attacker.value.getSquads(),
      defender: defender.value.getSquads(),
      wall: wallLevel.value,
    });
  }
  catch (err) {
    handleError(err);
  }
}

function clear() {
  attacker.value = ArmyPersonnelImpl.createEmpty();
  defender.value = ArmyPersonnelImpl.createEmpty();
}
</script>

<template>
  <div class="w-full min-w-max flex flex-col gap-4 xl:flex-row-reverse xl:justify-end">
    <BattleResultTable v-if="result" :result />
    <div class="w-full flex flex-col gap-4 xl:max-w-max">
      <PersonnelTable
        v-model:attacker="attacker"
        v-model:defender="defender"
        v-model:wall="wallLevel"
      />

      <div class="grid grid-cols-2 items-center justify-start gap-4 max-w-max">
        <Button variant="default" :disabled="!canSimulate" @click="simulate">
          <span>{{ t('calculate') }}</span>
        </Button>
        <Button variant="secondary" @click="clear">
          <span>{{ t('clear') }}</span>
        </Button>
      </div>
    </div>
  </div>
</template>
