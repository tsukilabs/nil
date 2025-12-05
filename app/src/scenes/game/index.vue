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
import Finder from '@/components/Finder.vue';
import Loading from '@/components/Loading.vue';
import { asyncRef, onCtrlKeyDown } from '@tb-dev/vue';
import { SidebarProvider } from '@tb-dev/vue-components';
import { defineGlobalCheats, DESKTOP } from '@/lib/global';
import { usePlayerReady } from '@/composables/player/usePlayerReady';

const { round } = NIL.round.refs();
const { isPlayerTurn, isPlayerReady, togglePlayerReady } = usePlayerReady();

const { state: isHost } = asyncRef(false, commands.isHost);

const [isSidebarOpen, toggleSidebar] = useToggle(false);
const [isFinderOpen, toggleFinder] = useToggle(false);

const lastSavedAt = ref<Option<RoundId>>();

if (__DESKTOP__) {
  onCtrlKeyDown(['b', 'B'], () => toggleSidebar());
  onCtrlKeyDown(['f', 'F'], () => toggleFinder());
  onCtrlKeyDown(['m', 'M'], () => go('continent'));
  onCtrlKeyDown(['s', 'S'], () => save());
  onCtrlKeyDown(' ', () => togglePlayerReady());
}

onMounted(() => defineGlobalCheats());

async function startRound() {
  if (isHost.value && round.value?.state.kind === 'idle') {
    await commands.startRound();
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
    <Sidebar :is-host :is-player-ready :last-saved-at @save="save" @leave="leaveGame" />

    <div class="bg-background/40 absolute inset-0 overflow-hidden">
      <Header
        :is-host
        :is-player-turn
        :is-player-ready
        class="bg-background absolute inset-x-0 top-0 h-20 sm:h-16 border-b px-4"
        @start-round="startRound"
        @toggle-player-ready="togglePlayerReady"
      />

      <div class="absolute inset-x-0 top-20 sm:top-16 bottom-10 overflow-hidden">
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

      <Finder v-if="DESKTOP" v-model:open="isFinderOpen" />
    </div>
  </SidebarProvider>
</template>
