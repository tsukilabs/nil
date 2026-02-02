<script setup lang="ts">
import type { NilServer } from '../composables/useServer';

defineProps<{ server: NilServer; }>();
</script>

<template>
  <p>Version: {{ server.version }}</p>

  <h2>Worlds</h2>
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
  </table>
</template>
