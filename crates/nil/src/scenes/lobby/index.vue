<script setup lang="ts">
import { ref } from 'vue';
import { leaveGame } from '@/core/game';
import { startRound } from '@/commands';
import { handleError } from '@/lib/error';
import { command } from '@/composables/command';
import { ListenerSet } from '@/lib/listener-set';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { Button, Card, Table, TableCell, TableRow } from '@tb-dev/vue';

const { state: serverAddr } = command('getServerAddr', null);
const { state: world } = command('getWorldState', null);
const { execute: getPlayers, state: players } = command('getPlayers', []);
const { state: isHost } = command('isHost', false);

const loading = ref(false);
const started = ref(false);

const listener = new ListenerSet();

// prettier-ignore
listener.event
  .onPlayerJoined(() => getPlayers())
  .onPlayerLeft(() => getPlayers())
  .onRoundUpdated((payload) => onRoundUpdated(payload.round));

async function start() {
  try {
    loading.value = true;
    await startRound();
    started.value = true;
  } catch (err) {
    handleError(err);
  } finally {
    loading.value = false;
  }
}

function leave() {
  listener.dispose();
  leaveGame().err();
}

async function onRoundUpdated(round: Round) {
  if (round.phase.kind === 'player') {
    try {
      loading.value = true;
      listener.dispose();
      await NIL.village.go();
    } catch (err) {
      handleError(err);
    } finally {
      loading.value = false;
    }
  }
}

function copyServerAddr() {
  if (serverAddr.value) {
    const addr = serverAddr.value.format();
    writeText(addr).err('Failed to copy server address');
  }
}
</script>

<template>
  <div>
    <div class="flex items-center justify-between p-4">
      <div class="flex flex-col items-start gap-2">
        <h1 v-if="world" class="text-xl font-bold">{{ world.name }}</h1>
        <h2 v-if="serverAddr" class="cursor-pointer text-sm" @click="copyServerAddr">
          {{ serverAddr.format() }}
        </h2>
      </div>
      <div class="flex items-center justify-end gap-2">
        <Button v-if="isHost" :disabled="loading || started" @click="start">
          {{ $t('start') }}
        </Button>
        <Button variant="secondary" :disabled="loading" @click="leave">
          {{ $t('leave') }}
        </Button>
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
