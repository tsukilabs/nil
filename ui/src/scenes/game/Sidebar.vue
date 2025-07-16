<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useTemplateRef } from 'vue';
import { asyncRef } from '@tb-dev/vue';
import * as commands from '@/commands';
import { onBeforeRouteUpdate } from 'vue-router';
import type { MaybePromise } from '@tb-dev/utils';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
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
} from '@tb-dev/vue-components';

const props = defineProps<{
  isHost: boolean;
  toggleSidebar: (value?: boolean) => boolean;
  onSave: () => MaybePromise<void>;
  onLeave: () => MaybePromise<void>;
}>();

const { t } = useI18n();

const { config } = NIL.world.refs();
const { round } = NIL.round.refs();

const { state: serverAddr } = asyncRef(null, commands.getServerAddr);

const sidebarFooter = useTemplateRef('sidebarFooterEl');
const onClickOutsideOptions: OnClickOutsideProps['options'] = {
  ignore: [sidebarFooter],
};

const closeSidebar = () => void props.toggleSidebar(false);

onBeforeRouteUpdate(closeSidebar);

function copyServerAddr() {
  if (serverAddr.value) {
    const addr = serverAddr.value.format();
    writeText(addr).err();
  }
}
</script>

<template>
  <Sidebar class="z-[var(--game-sidebar-z-index)]">
    <SidebarHeader>
      <div class="flex flex-col items-center">
        <h1 v-if="config" class="font-nil text-lg">{{ config.name }}</h1>
        <h2
          v-if="serverAddr"
          class="text-muted-foreground cursor-pointer text-sm"
          @click="copyServerAddr"
        >
          {{ serverAddr.format() }}
        </h2>
      </div>
    </SidebarHeader>

    <SidebarContent>
      <div v-on-click-outside="[closeSidebar, onClickOutsideOptions]" class="size-full">
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
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
        class="grid grid-cols-2 items-center justify-center gap-4 px-6 pb-4"
      >
        <Button size="sm" :disabled="!isHost || round?.phase.kind === 'idle'" @click="onSave">
          <span>{{ t('save') }}</span>
        </Button>
        <Button variant="destructive" size="sm" @click="onLeave">
          <span>{{ t('leave') }}</span>
        </Button>
      </div>
    </SidebarFooter>
  </Sidebar>
</template>
