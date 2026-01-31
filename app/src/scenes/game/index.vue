<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { ref } from 'vue';
import { go } from '@/router';
import Footer from './Footer.vue';
import Header from './Header.vue';
import Sidebar from './Sidebar.vue';
import * as commands from '@/commands';
import { DESKTOP } from '@/lib/global';
import { leaveGame } from '@/core/game';
import { useToggle } from '@vueuse/core';
import { handleError } from '@/lib/error';
import Finder from '@/components/Finder.vue';
import Loading from '@/components/Loading.vue';
import { saveLocalGame } from '@/core/savedata';
import { SidebarProvider } from '@tb-dev/vue-components';
import { RemoteWorldImpl } from '@/core/model/remote-world';
import { usePlayerReady } from '@/composables/player/usePlayerReady';
import { asyncComputed, asyncRef, onCtrlKeyDown } from '@tb-dev/vue';

const { worldId } = NIL.world.refs();
const { round } = NIL.round.refs();
const { player } = NIL.player.refs();

const { isPlayerTurn, isPlayerReady, togglePlayerReady } = usePlayerReady();

const { state: isHost } = asyncRef(false, commands.isHost);
const { state: isLocal } = asyncRef(false, commands.isLocal);

const [isSidebarOpen, toggleSidebar] = useToggle(false);
const [isFinderOpen, toggleFinder] = useToggle(false);

const lastSavedAt = ref<Option<RoundId>>();

const isRemoteCreatedBySelf = asyncComputed(false, async () => {
  // We can’t use `isLocal` because, by the time the computed runs
  // for the first time, it may not have been loaded yet.
  if (worldId.value && player.value && await commands.isRemote()) {
    return RemoteWorldImpl.wasCreatedBy(worldId.value, player.value.id);
  }
  else {
    return false;
  }
});

if (__DESKTOP__) {
  onCtrlKeyDown(['b', 'B'], () => toggleSidebar());
  onCtrlKeyDown(['f', 'F'], () => toggleFinder());
  onCtrlKeyDown(['m', 'M'], () => go('continent'));
  onCtrlKeyDown(['s', 'S'], () => save());
  onCtrlKeyDown(' ', () => togglePlayerReady());
}

async function startRound() {
  if (
    (isHost.value || isRemoteCreatedBySelf.value) &&
    round.value?.state.kind === 'idle'
  ) {
    await commands.startRound();
  }
}

async function save() {
  if (
    isHost.value &&
    isLocal.value &&
    round.value?.state.kind !== 'idle' &&
    round.value?.id !== lastSavedAt.value
  ) {
    try {
      await saveLocalGame();
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
    <Sidebar
      :is-host
      :is-local
      :is-player-ready
      :last-saved-at
      @save="save"
      @leave="leaveGame"
    />

    <div class="bg-background/40 absolute inset-0 overflow-hidden">
      <Header
        :is-host
        :is-player-turn
        :is-player-ready
        :is-remote-created-by-self
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
