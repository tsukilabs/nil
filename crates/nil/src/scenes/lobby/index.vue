<script setup lang="ts">
import { leaveGame } from '@/core/game';
import { command } from '@/composables/command';
import { ListenerSet } from '@/lib/listener-set';
import { Button, Card, Table, TableCell, TableRow } from '@/components';

const serverAddr = command('getServerAddr', null);
const world = command('getWorldState', null);
const players = command('getPlayers', []);
const isHost = command('isHost', false);

const set = new ListenerSet();
set.event.onPlayerJoined(() => players.execute());
</script>

<template>
  <div>
    <div class="flex items-center justify-between p-4">
      <div class="flex flex-col items-start gap-2">
        <h1 v-if="world" class="text-xl font-bold">{{ world.name }}</h1>
        <h2 v-if="serverAddr" class="text-sm">{{ serverAddr.format() }}</h2>
      </div>
      <div class="flex items-center justify-end gap-2">
        <Button v-if="isHost">{{ $t('start') }}</Button>
        <Button variant="secondary" @click="leaveGame">{{ $t('leave') }}</Button>
      </div>
    </div>
    <div class="px-2">
      <Card>
        <Table>
          <template #header>
            <TableRow>
              <TableCell>{{ $t('player') }}</TableCell>
            </TableRow>
          </template>
          <TableRow v-for="player of players" :key="player.id">
            <TableCell>{{ player.id }}</TableCell>
          </TableRow>
        </Table>
      </Card>
    </div>
  </div>
</template>
