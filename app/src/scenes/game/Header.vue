<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from "@/router";
import { Button } from "@ui/button";
import { useToggle } from "@vueuse/core";
import { Separator } from "@ui/separator";
import Round from "@/scenes/game/Round.vue";
import { useBreakpoints } from "@tb-dev/vue";
import { SidebarTrigger } from "@ui/sidebar";
import { ChevronDownIcon } from "@lucide/vue";
import Resources from "@/scenes/game/Resources.vue";
import HeaderCities from "@/scenes/game/HeaderCities.vue";
import ButtonIcon from "@/components/button/ButtonIcon.vue";

defineProps<{
  isHost: boolean;
  isPlayerReady: boolean;
  isPlayerTurn: boolean;
  isRemoteCreatedBySelf: boolean;
  onStartRound: () => Promise<void>;
  onTogglePlayerReady: () => Promise<void>;
}>();

const { city: currentCity } = NIL.city.refs();
const { cities: playerCities } = NIL.player.refs();

const { sm } = useBreakpoints();

const [isCitySelectOpen, toggleCitySelect] = useToggle(false);
</script>

<template>
  <header class="flex flex-col items-center justify-center gap-2 overflow-hidden">
    <div class="w-full flex items-center justify-between">
      <div class="max-w-3/5 flex items-center shrink-0 gap-2">
        <SidebarTrigger class="-ml-1" />
        <Separator orientation="vertical" class="data-[orientation=vertical]:h-4" />
        <div class="flex items-center gap-0.5">
          <Button
            v-if="currentCity"
            variant="ghost"
            role="link"
            tabindex="0"
            class="px-1 py-2 text-base lg:text-lg"
            @click.stop="() => go('city')"
            @keydown.enter.stop="() => go('city')"
          >
            <span v-if="sm">{{ currentCity.formatNameWithCoord() }}</span>
            <span v-else>{{ currentCity.formatCoord() }}</span>
          </Button>

          <ButtonIcon
            :icon="ChevronDownIcon"
            size="icon-sm"
            @click="() => void toggleCitySelect()"
          />
        </div>
      </div>

      <div class="flex items-center">
        <Round
          :is-host
          :is-player-ready
          :is-player-turn
          :is-remote-created-by-self
          @start-round="onStartRound"
          @toggle-player-ready="onTogglePlayerReady"
        />
      </div>
    </div>

    <div class="w-full flex items-center justify-center">
      <Resources v-if="!sm" />
    </div>

    <HeaderCities
      v-if="currentCity && playerCities.length > 0"
      v-model:open="isCitySelectOpen"
      :current-city
      :player-cities
    />
  </header>
</template>
