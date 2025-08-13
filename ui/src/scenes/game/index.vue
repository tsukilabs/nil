<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import { onMounted } from 'vue';
import Header from './Header.vue';
import Footer from './Footer.vue';
import Sidebar from './Sidebar.vue';
import * as commands from '@/commands';
import { leaveGame } from '@/core/game';
import { useToggle } from '@vueuse/core';
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

onCtrlKeyDown(['b', 'B'], () => toggleSidebar());
onCtrlKeyDown(['f', 'F'], () => toggleFinder());
onCtrlKeyDown(['m', 'M'], () => go('continent'));
onCtrlKeyDown(['s', 'S'], () => save());
onCtrlKeyDown(' ', () => endTurn());

onMounted(() => defineGlobalCheats());

function startRound() {
  if (isHost.value && round.value?.state.kind === 'idle') {
    commands.startRound().err();
  }
}

function endTurn() {
  if (isPlayerTurn.value) {
    commands.endTurn().err();
  }
}

function save() {
  if (isHost.value && round.value?.state.kind !== 'idle') {
    saveGame().err();
  }
}
</script>

<template>
  <SidebarProvider v-model:open="isSidebarOpen">
    <Sidebar :is-host :toggle-sidebar @save="save" @leave="leaveGame" />

    <div class="bg-background/40 absolute inset-0 overflow-hidden">
      <Header
        :is-host
        class="bg-background absolute inset-x-0 top-0 h-16 border-b px-4"
        @start-round="startRound"
        @end-turn="endTurn"
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

      <Finder v-model:open="isFinderOpen" />
    </div>
  </SidebarProvider>
</template>
