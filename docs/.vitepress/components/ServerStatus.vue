<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import type { NilServer } from '../composables/useServer';

defineProps<{
  server: NilServer;
}>();
</script>

<template>
  <table>
    <thead>
      <tr>
        <th>Name</th>
        <th>Round</th>
        <th>Players</th>
      </tr>
    </thead>

    <tbody>
      <tr v-for="world of server.worlds" :key="world.config.id">
        <td>{{ world.config.name }}</td>
        <td>{{ world.currentRound }}</td>
        <td>{{ world.totalPlayers }}</td>
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
.version {
  text-align: right !important;
}
</style>
