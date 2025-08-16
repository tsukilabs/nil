<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { asyncRef } from '@tb-dev/vue';
import * as commands from '@/commands';
import RoundState from './RoundState.vue';
import { nextTick, useTemplateRef } from 'vue';
import { onBeforeRouteUpdate } from 'vue-router';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import { type OnClickOutsideProps, vOnClickOutside } from '@vueuse/components';
import {
  Button,
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from '@tb-dev/vue-components';

defineProps<{
  isHost: boolean;
  onSave: () => MaybePromise<void>;
  onLeave: () => MaybePromise<void>;
}>();

const { t } = useI18n();

const sidebar = useSidebar();

const { config } = NIL.world.refs();
const { round } = NIL.round.refs();
const { player } = NIL.player.refs();

const { state: serverAddr } = asyncRef(null, commands.getServerAddr);

const sidebarHeader = useTemplateRef('sidebarHeaderEl');
const sidebarFooter = useTemplateRef('sidebarFooterEl');
const onClickOutsideOptions: OnClickOutsideProps['options'] = {
  ignore: [sidebarHeader, sidebarFooter],
};

const { sm } = useBreakpoints();

onBeforeRouteUpdate(closeSidebar);

async function closeSidebar() {
  await nextTick();
  if (sidebar.isMobile.value) {
    sidebar.setOpenMobile(false);
  }
  else {
    sidebar.setOpen(false);
  }
}

function copyServerAddr() {
  if (serverAddr.value) {
    const addr = serverAddr.value.format();
    writeText(addr).err();
  }
}
</script>

<template>
  <Sidebar class="z-[var(--game-sidebar-z-index)] select-none">
    <SidebarHeader>
      <div ref="sidebarHeaderEl" class="flex flex-col items-center overflow-hidden pt-4">
        <h1 v-if="config" class="font-nil text-lg break-all text-center">
          {{ config.name }}
        </h1>
        <h2
          v-if="serverAddr"
          class="text-muted-foreground cursor-pointer text-sm"
          @click="copyServerAddr"
        >
          {{ serverAddr.format() }}
        </h2>

        <RoundState v-if="!sm && player && round?.state.kind === 'waiting'" class="mt-4" />
      </div>
    </SidebarHeader>

    <SidebarContent>
      <div v-on-click-outside="[closeSidebar, onClickOutsideOptions]" class="size-full">
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton as-child>
                  <RouterLink :to="{ name: 'continent' satisfies Scene }">
                    {{ t('continent') }}
                  </RouterLink>
                </SidebarMenuButton>
              </SidebarMenuItem>

              <SidebarMenuItem>
                <SidebarMenuButton as-child>
                  <RouterLink :to="{ name: 'script' satisfies GameScene }">
                    {{ t('script', 2) }}
                  </RouterLink>
                </SidebarMenuButton>
              </SidebarMenuItem>

              <SidebarMenuItem>
                <SidebarMenuButton as-child>
                  <RouterLink :to="{ name: 'settings' satisfies Scene }">
                    {{ t('settings') }}
                  </RouterLink>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </div>
    </SidebarContent>

    <SidebarFooter>
      <div
        ref="sidebarFooterEl"
        class="grid grid-cols-2 items-center justify-center gap-4"
      >
        <Button size="sm" :disabled="!isHost || round?.state.kind === 'idle'" @click="onSave">
          <span>{{ t('save') }}</span>
        </Button>
        <Button variant="destructive" size="sm" @click="onLeave">
          <span>{{ t('leave') }}</span>
        </Button>
      </div>
    </SidebarFooter>
  </Sidebar>
</template>
