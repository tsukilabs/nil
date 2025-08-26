<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import Header from './Header.vue';
import Footer from './Footer.vue';
import Sidebar from './Sidebar.vue';
import { onMounted, ref } from 'vue';
import * as commands from '@/commands';
import { leaveGame } from '@/core/game';
import { useToggle } from '@vueuse/core';
import { handleError } from '@/lib/error';
import { saveGame } from '@/core/savedata';
import Loading from '@/components/Loading.vue';
import { defineGlobalCheats } from '@/lib/global';
import Finder from '@/components/finder/Finder.vue';
import { asyncRef, onCtrlKeyDown } from '@tb-dev/vue';
import { SidebarProvider } from '@tb-dev/vue-components';
import { usePlayerTurn } from '@/composables/player/usePlayerTurn';

const { round } = NIL.round.refs();

const isPlayerTurn = usePlayerTurn();
const { state: isHost } = asyncRef(false, commands.isHost);

const [isSidebarOpen, toggleSidebar] = useToggle(false);
const [isFinderOpen, toggleFinder] = useToggle(false);

const lastSavedAt = ref<Option<RoundId>>();

const desktop = globalThis.__DESKTOP__;

onCtrlKeyDown(['b', 'B'], () => toggleSidebar(), { enabled: desktop });
onCtrlKeyDown(['f', 'F'], () => toggleFinder(), { enabled: desktop });
onCtrlKeyDown(['m', 'M'], () => go('continent'), { enabled: desktop });
onCtrlKeyDown(['s', 'S'], () => save(), { enabled: desktop });
onCtrlKeyDown(' ', () => finishTurn(), { enabled: desktop });

onMounted(() => defineGlobalCheats());

async function startRound() {
  if (isHost.value && round.value?.state.kind === 'idle') {
    await commands.startRound();
  }
}

async function finishTurn() {
  if (isPlayerTurn.value) {
    await commands.endTurn();
  }
}

async function save() {
  if (
    isHost.value &&
    round.value?.state.kind !== 'idle' &&
    round.value?.id !== lastSavedAt.value
  ) {
    try {
      await saveGame();
      lastSavedAt.value = round.value?.id;
    }
    catch (err) {
      handleError(err);
    }
  }
}
</script>

<template>
  <SidebarProvider v-model:open="isSidebarOpen">
    <Sidebar :is-host :last-saved-at @save="save" @leave="leaveGame" />

    <div class="bg-background/40 absolute inset-0 overflow-hidden">
      <Header
        :is-host
        class="bg-background absolute inset-x-0 top-0 h-16 border-b px-4"
        @start-round="startRound"
        @finish-turn="finishTurn"
      />

      <div class="absolute inset-x-0 top-16 bottom-10 overflow-hidden">
        <RouterView #default="{ Component }">
          <template v-if="Component">
            <Suspense>
              <component :is="Component" />
              <template #fallback>
                <Loading class="absolute inset-0" />
              </template>
            </Suspense>
          </template>
        </RouterView>
      </div>

      <Footer class="bg-background absolute inset-x-0 bottom-0 h-10 border-t px-6" />

      <Finder v-if="desktop" v-model:open="isFinderOpen" />
    </div>
  </SidebarProvider>
</template>
