<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import Header from './Header.vue';
import Footer from './Footer.vue';
import * as commands from '@/commands';
import { useToggle } from '@vueuse/core';
import { onCtrlKeyDown } from '@tb-dev/vue';
import { onBeforeRouteUpdate } from 'vue-router';
import { leaveGame, saveGame } from '@/core/game';
import { defineGlobalCheats } from '@/lib/console';
import { nextTick, onMounted, useTemplateRef } from 'vue';
import { Button, Loading, Sidebar } from '@tb-dev/vue-components';
import { type OnClickOutsideProps, vOnClickOutside } from '@vueuse/components';

const { isPlayerTurn } = NIL.round.refs();

const [isSidebarOpen, toggleSidebar] = useToggle(false);
const closeSidebar = () => void toggleSidebar(false);

const sidebarFooter = useTemplateRef('sidebarFooterEl');
const onClickOutsideOptions: OnClickOutsideProps['options'] = {
  ignore: [sidebarFooter],
};

onCtrlKeyDown(['b', 'B'], () => toggleSidebar());
onCtrlKeyDown(['m', 'M'], () => go('continent'));
onCtrlKeyDown(['s', 'S'], () => saveGame());
onCtrlKeyDown(' ', () => endTurn());

onBeforeRouteUpdate(closeSidebar);

onMounted(() => defineGlobalCheats());

async function endTurn() {
  await nextTick();
  if (isPlayerTurn.value) {
    commands.endTurn().err();
  }
}
</script>

<template>
  <Sidebar v-model:open="isSidebarOpen">
    <div class="bg-muted/40 absolute inset-0 overflow-hidden">
      <Header
        class="bg-background absolute inset-x-0 top-0 h-16 border-b px-4"
        @turn-end="endTurn"
      />

      <div class="absolute inset-x-0 top-16 bottom-10 overflow-hidden">
        <RouterView #default="{ Component }">
          <template v-if="Component">
            <Suspense>
              <component :is="Component" />
              <template #fallback>
                <div class="absolute inset-0 flex items-center justify-center">
                  <Loading />
                </div>
              </template>
            </Suspense>
          </template>
        </RouterView>
      </div>
      <Footer class="bg-background absolute inset-x-0 bottom-0 h-10 border-t px-6" />
    </div>

    <template #content>
      <div v-on-click-outside="[closeSidebar, onClickOutsideOptions]" class="size-full p-4">
        <RouterLink :to="{ name: 'script' }">Scripts</RouterLink>
      </div>
    </template>

    <template #footer>
      <div
        ref="sidebarFooterEl"
        class="grid grid-cols-2 items-center justify-center gap-4 px-6 pb-4"
      >
        <Button size="sm" @click="saveGame">{{ $t('save') }}</Button>
        <Button variant="destructive" size="sm" @click="leaveGame">{{ $t('leave') }}</Button>
      </div>
    </template>
  </Sidebar>
</template>
