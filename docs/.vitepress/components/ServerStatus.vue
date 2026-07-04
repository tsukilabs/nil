<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import type { Server } from "../composables/useServer";

defineProps<{
  server: Server;
}>();
</script>

<template>
  <table>
    <thead>
      <tr>
        <th>Name</th>
        <th>Round</th>
        <th>Players</th>
        <th :class="$style['maybe-hidden']">Size</th>
        <th :class="$style['maybe-hidden']">Speed</th>
        <th :class="$style['maybe-hidden']">Unit speed</th>
      </tr>
    </thead>

    <tbody>
      <tr v-for="world of server.worlds" :key="world.config.id">
        <td>{{ world.config.name }}</td>
        <td>{{ world.currentRound }}</td>
        <td>{{ world.totalPlayers }}</td>
        <td :class="$style['maybe-hidden']">{{ world.continentSize }}</td>
        <td :class="$style['maybe-hidden']">{{ world.config.speed }}</td>
        <td :class="$style['maybe-hidden']">{{ world.config.unitSpeed }}</td>
      </tr>
    </tbody>

    <tfoot>
      <tr>
        <th scope="row" colspan="2" :class="$style.version">Version</th>
        <td>{{ server.version }}</td>
      </tr>
    </tfoot>
  </table>
</template>

<style module>
.maybe-hidden {
  @media (width < 64rem) {
    display: none;
  }
}

.version {
  text-align: right !important;
}
</style>
