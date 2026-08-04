<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import * as commands from "@/commands";
import { handleError } from "@/lib/error";
import type { Option } from "@tb-dev/utils";
import { computed, nextTick, ref } from "vue";
import { throttle } from "es-toolkit/function";
import { Button } from "@/components/ui/button";
import { ListenerSet } from "@/lib/listener-set";
import type { Ruler } from "@tsukilabs/nil-bindings";
import { ResourcesImpl } from "@/core/model/resources";
import { onKeyDown, useBreakpoints } from "@tb-dev/vue";
import { useRulers } from "@/composables/ruler/useRulers";
import Rulers from "@/scenes/game/market/send/Rulers.vue";
import { useMarket } from "@/composables/market/useMarket";
import { usePlayerTurn } from "@/composables/player/usePlayerTurn";
import ResourcesGrid from "@/scenes/game/market/send/ResourcesGrid.vue";

const { t } = useI18n();

const { market, load: loadMarket, loading: isLoadingMarket } = useMarket();
const { rulers, load: loadRulers, loading: isLoadingRulers } = useRulers();

const recipient = ref<Option<Ruler>>();
const resources = ref(ResourcesImpl.splat(0));

const isPlayerTurn = usePlayerTurn();

const canSend = computed(() => {
  return (
    isPlayerTurn.value &&
    market.value &&
    rulers.value.length > 1 &&
    !isLoadingMarket.value &&
    !isLoadingRulers.value &&
    recipient.value &&
    !resources.value.isEmpty()
  );
});

const { sm } = useBreakpoints();

const listener = new ListenerSet();
listener.event.onMarket(throttle(loadMarket, 1000));

if (__DESKTOP__) {
  onKeyDown("F5", throttle(load, 1000));
}

async function send() {
  await nextTick();
  if (canSend.value && market.value && recipient.value) {
    try {
      await commands.sendResources(
        recipient.value,
        resources.value.toJSON(),
      );

      resources.value = ResourcesImpl.splat(0);
    }
    catch (err) {
      handleError(err);
    }
  }
}

async function load() {
  await Promise.all([loadMarket(), loadRulers()]);
}

function clear() {
  recipient.value = null;
  resources.value = ResourcesImpl.splat(0);
}
</script>

<template>
  <div class="size-full flex flex-col gap-4">
    <div class="w-full lg:min-w-max lg:max-w-1/2 grid grid-cols-1 gap-8">
      <Rulers v-model="recipient" :rulers :loading="isLoadingRulers" />
      <ResourcesGrid v-model="resources" :market-fee="market?.fee ?? 0" />
    </div>

    <div class="max-sm:w-full sm:max-w-max grid grid-cols-2 items-center justify-center sm:justify-start gap-4">
      <Button
        variant="default"
        :size="sm ? 'default' : 'sm'"
        :disabled="!canSend"
        @click.stop="send"
      >
        <span>{{ t("send") }}</span>
      </Button>
      <Button
        variant="secondary"
        :size="sm ? 'default' : 'sm'"
        @click.stop="clear"
      >
        <span>{{ t("clear") }}</span>
      </Button>
    </div>
  </div>
</template>
