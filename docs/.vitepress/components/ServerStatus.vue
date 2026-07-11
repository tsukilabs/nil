<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { LockIcon } from "@lucide/vue";
import { useBreakpoints } from "@tb-dev/vue";
import type { Server } from "../composables/useServer";

defineProps<{
  server: Server;
}>();

const { lg } = useBreakpoints();
</script>

<template>
  <table>
    <thead>
      <tr>
        <th>Name</th>
        <th>Round</th>
        <th>Players</th>
        <th :class="lg ? null : $style.hidden">Size</th>
        <th :class="lg ? null : $style.hidden">Speed</th>
        <th :class="lg ? null : $style.hidden">Unit speed</th>
      </tr>
    </thead>

    <tbody>
      <tr v-for="world of server.worlds" :key="world.config.id">
        <td>
          <span>{{ world.config.name }}</span>
          <LockIcon v-if="world.hasPassword" :class="$style.icon" />
        </td>
        <td>{{ world.currentRound }}</td>
        <td>{{ world.totalPlayers }}</td>
        <td :class="lg ? null : $style.hidden">{{ world.continentSize }}</td>
        <td :class="lg ? null : $style.hidden">{{ world.config.speed }}</td>
        <td :class="lg ? null : $style.hidden">{{ world.config.unitSpeed }}</td>
      </tr>
    </tbody>

    <tfoot>
      <tr>
        <th scope="row" :colspan="lg ? 5 : 2" :class="$style.version">Version</th>
        <td>{{ server.version }}</td>
      </tr>
    </tfoot>
  </table>
</template>

<style module>
.hidden {
  display: none;
}

.version {
  text-align: right !important;
}

.icon {
  display: inline-block;
  width: 0.75rem;
  height: 0.75rem;
  margin-left: 0.5rem;
}
</style>
