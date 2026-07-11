<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from "vue";
import { useBreakpoints } from "@tb-dev/vue";
import Resources from "@/scenes/game/Resources.vue";
import type { GameScene } from "@/types/scene/game";
import FooterChat from "@/scenes/game/FooterChat.vue";
import ButtonIcon from "@/components/button/ButtonIcon.vue";
import ButtonBadge from "@/components/button/ButtonBadge.vue";
import { EarthIcon, ScrollTextIcon, SwordsIcon } from "@lucide/vue";

const { player } = NIL.player.refs();
const { military } = NIL.military.refs();
const { unread: unreadReports } = NIL.report.refs();

const { sm } = useBreakpoints();

const hasIncomingAttack = computed(() => {
  if (player.value && military.value) {
    return military.value.maneuvers
      .values()
      .some((maneuver) => {
        return (
          maneuver.kind === "attack" &&
          maneuver.direction === "going" &&
          maneuver.state.kind === "pending" &&
          player.value?.hasCity(maneuver.destination)
        );
      });
  }
  else {
    return false;
  }
});
</script>

<template>
  <footer class="flex items-center justify-between gap-4 overflow-hidden">
    <Resources v-if="sm" />
    <div class="flex size-full min-w-max items-center justify-end gap-2 sm:gap-4">
      <RouterLink :to="{ name: 'war-room' satisfies GameScene }">
        <ButtonBadge :icon="SwordsIcon" :show-badge="hasIncomingAttack" />
      </RouterLink>

      <RouterLink :to="{ name: 'continent' satisfies GameScene }">
        <ButtonIcon :icon="EarthIcon" />
      </RouterLink>

      <RouterLink :to="{ name: 'report' satisfies GameScene }">
        <ButtonBadge :icon="ScrollTextIcon" :show-badge="unreadReports.size > 0" />
      </RouterLink>

      <FooterChat />
    </div>
  </footer>
</template>
