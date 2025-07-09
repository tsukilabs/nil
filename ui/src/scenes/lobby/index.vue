<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import * as commands from '@/commands';
import { asyncRef } from '@tb-dev/vue';
import { leaveGame } from '@/core/game';
import { Chat } from '@/components/chat';
import { handleError } from '@/lib/error';
import { computed, nextTick, ref } from 'vue';
import { LobbyImpl } from '@/core/model/lobby';
import { ListenerSet } from '@/lib/listener-set';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import {
  Button,
  Checkbox,
  type CheckboxValue,
  Table,
  TableCell,
  TableHead,
  TableRow,
  toBooleanCheckboxValue,
} from '@tb-dev/vue-components';

const { t } = useI18n();

const { config } = NIL.world.refs();
const { player: currentPlayer } = NIL.player.refs();

const { state: lobby } = asyncRef(null, LobbyImpl.load.bind(LobbyImpl));
const { execute: updatePlayers, state: players } = asyncRef([], commands.getPlayers);
const { state: serverAddr } = asyncRef(null, commands.getServerAddr);
const { state: isHost } = asyncRef(false, commands.isHost);

const loading = ref(false);
const started = ref(false);

const canStart = computed(() => {
  return !started.value && !loading.value && isHostReady();
});

const listener = new ListenerSet();
listener.event
  .onGuestLeft(() => updatePlayers())
  .onLobbyUpdated(onLobbyUpdated)
  .onPlayerSpawned(() => updatePlayers())
  .onRoundUpdated(onRoundUpdated);

async function start() {
  await nextTick();
  if (isHost.value && canStart.value) {
    try {
      loading.value = true;
      await updatePlayers();

      const promises: Promise<unknown>[] = [];
      for (const player of players.value) {
        if (lobby.value?.isReady(player.id)) {
          if (player.status === 'guest') {
            promises.push(commands.spawnPlayerVillage(player.id));
          } else if (player.status === 'inactive') {
            promises.push(commands.setPlayerStatus(player.id, 'active'));
          }
        }
      }

      await Promise.all(promises);
      await commands.startRound();
      await NIL.update();
      started.value = true;
    } catch (err) {
      handleError(err);
    } finally {
      loading.value = false;
    }
  }
}

function leave() {
  listener.dispose();
  leaveGame().err();
}

async function setLobbyReady(playerId: PlayerId, value: CheckboxValue) {
  if (currentPlayer.value?.id === playerId) {
    await commands.setLobbyReady(playerId, toBooleanCheckboxValue(value));
  }
}

function onLobbyUpdated(payload: LobbyUpdatedPayload) {
  lobby.value = LobbyImpl.create(payload.lobby);
}

async function onRoundUpdated({ round }: RoundUpdatedPayload) {
  if (round.phase.kind === 'player') {
    try {
      loading.value = true;
      await NIL.update();
      await NIL.village.go({ timeout: 500, keepTrying: true });
      listener.dispose();
    } catch (err) {
      handleError(err);
    } finally {
      loading.value = false;
    }
  } else {
    started.value = false;
  }
}

function copyServerAddr() {
  if (serverAddr.value) {
    const addr = serverAddr.value.format();
    writeText(addr).err('Failed to copy server address');
  }
}

function isHostReady() {
  if (isHost.value && lobby.value && currentPlayer.value?.id) {
    return lobby.value.isReady(currentPlayer.value.id);
  }

  return false;
}
</script>

<template>
  <div class="flex size-full flex-col">
    <div class="flex items-center justify-between p-4">
      <div class="flex flex-col items-start gap-2">
        <h1 v-if="config?.name" class="text-xl font-bold">{{ config.name }}</h1>
        <h2 v-if="serverAddr" class="cursor-pointer text-sm" @click="copyServerAddr">
          {{ serverAddr.format() }}
        </h2>
      </div>
      <div class="flex items-center justify-end gap-2">
        <Button v-if="isHost" :disabled="!canStart" @click="start">
          {{ t('start') }}
        </Button>
        <Button variant="secondary" :disabled="loading" @click="leave">
          {{ t('leave') }}
        </Button>
      </div>
    </div>

    <div v-if="lobby" class="flex size-full justify-between gap-12 overflow-hidden px-2">
      <Table header-class="sticky top-0 z-50">
        <template #header>
          <TableRow class="bg-background hover:bg-background">
            <TableHead>
              <span>{{ t('player') }}</span>
            </TableHead>
            <TableHead class="w-20">
              <span>{{ t('ready') }}</span>
            </TableHead>
          </TableRow>
        </template>

        <TableRow v-for="player of players" :key="player.id">
          <TableCell>
            <span>{{ player.id }}</span>
          </TableCell>
          <TableCell>
            <Checkbox
              :disabled="player.id !== currentPlayer?.id"
              :model-value="lobby.isReady(player.id)"
              @update:model-value="(ready) => setLobbyReady(player.id, ready)"
            />
          </TableCell>
        </TableRow>
      </Table>

      <div class="flex h-full w-2/5 max-w-[500px] min-w-[250px] flex-col gap-4 pb-8">
        <div class="h-8">
          <h1 class="text-xl font-bold">{{ t('chat') }}</h1>
        </div>
        <Chat class="size-full" />
      </div>
    </div>
  </div>
</template>
