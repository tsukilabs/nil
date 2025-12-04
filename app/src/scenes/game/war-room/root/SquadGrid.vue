<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import SquadGridItem from './SquadGridItem.vue';

defineProps<{
  available: ArmyPersonnel;
}>();

const personnel = defineModel<ArmyPersonnel>({ required: true });
</script>

<template>
  <div class="max-w-max grid grid-cols-2 md:grid-cols-4 gap-2">
    <SquadGridItem v-model="personnel.pikeman" :available="available.pikeman.size" />
    <SquadGridItem v-model="personnel.swordsman" :available="available.swordsman.size" />
    <SquadGridItem v-model="personnel.axeman" :available="available.axeman.size" />
    <SquadGridItem v-model="personnel.archer" :available="available.archer.size" />
    <SquadGridItem v-model="personnel.lightCavalry" :available="available.lightCavalry.size" />
    <SquadGridItem v-model="personnel.heavyCavalry" :available="available.heavyCavalry.size" />
  </div>
</template>
