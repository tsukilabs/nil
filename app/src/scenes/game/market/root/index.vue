<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { handleError } from "@/lib/error";
import { useI18n } from "@tsukilabs/nil-i18n";
import { computed, nextTick, ref } from "vue";
import { Button } from "@/components/ui/button";
import { ListenerSet } from "@/lib/listener-set";
import Food from "@/components/resources/Food.vue";
import Iron from "@/components/resources/Iron.vue";
import Wood from "@/components/resources/Wood.vue";
import Stone from "@/components/resources/Stone.vue";
import { ResourcesImpl } from "@/core/model/resources";
import { onKeyDown, useBreakpoints } from "@tb-dev/vue";
import { useMarket } from "@/composables/market/useMarket";
import { usePlayerTurn } from "@/composables/player/usePlayerTurn";
import ResourcesGrid from "@/scenes/game/market/root/ResourcesGrid.vue";
import { Table, TableBody, TableCell, TableHead, TableRow } from "@ui/table";

const { t } = useI18n();

const {
  market,
  throttledLoad,
  loading,
  buyResources,
  sellResources,
} = useMarket();

const { player } = NIL.player.refs();
const isPlayerTurn = usePlayerTurn();

const resources = ref(ResourcesImpl.splat(0));

const { sm } = useBreakpoints();

const canBuy = computed(() => {
  return (
    isPlayerTurn.value &&
    !loading.value &&
    !resources.value.isEmpty() &&
    market.value?.hasResourcesInVault(resources.value)
  );
});

const canSell = computed(() => {
  return (
    isPlayerTurn.value &&
    market.value &&
    !loading.value &&
    !resources.value.isEmpty() &&
    player.value?.hasResources(resources.value)
  );
});

const vaultResourceIconClass = computed(() => !sm.value ? "size-2 min-h-2 min-w-2" : null);
const vaultResourceTextClass = computed(() => !sm.value ? "text-sm" : null);

const listener = new ListenerSet();
listener.event
  .onMarket(throttledLoad)
  .onRound(throttledLoad);

if (__DESKTOP__) {
  onKeyDown("F5", throttledLoad);
}

async function buy() {
  await nextTick();
  if (canBuy.value) {
    try {
      await buyResources(resources.value);
      resources.value = ResourcesImpl.splat(0);
    }
    catch (err) {
      handleError(err);
    }
  }
}

async function sell() {
  await nextTick();
  if (canSell.value) {
    try {
      await sellResources(resources.value);
      resources.value = ResourcesImpl.splat(0);
    }
    catch (err) {
      handleError(err);
    }
  }
}

function clear() {
  resources.value = ResourcesImpl.splat(0);
}
</script>

<template>
  <div class="size-full flex flex-col gap-4">
    <div class="w-full lg:min-w-max lg:max-w-1/2 grid grid-cols-1 gap-8">
      <Table>
        <TableBody>
          <TableRow class="hover:bg-card">
            <TableHead>
              <span>{{ t("gold") }}</span>
            </TableHead>
            <TableCell>
              <span>{{ player?.gold ?? 0 }}</span>
            </TableCell>
          </TableRow>
          <TableRow class="hover:bg-card">
            <TableHead>
              <span>{{ t("market.vault") }}</span>
            </TableHead>
            <TableCell>
              <div class="flex justify-start items-center gap-2 pr-4">
                <Wood
                  :amount="market?.vault?.wood"
                  :icon-class="vaultResourceIconClass"
                  :text-class="vaultResourceTextClass"
                  class="md:pr-8"
                />
                <Stone
                  :amount="market?.vault?.stone"
                  :icon-class="vaultResourceIconClass"
                  :text-class="vaultResourceTextClass"
                  class="md:pr-8"
                />
                <Iron
                  :amount="market?.vault?.iron"
                  :icon-class="vaultResourceIconClass"
                  :text-class="vaultResourceTextClass"
                  class="md:pr-8"
                />
                <Food
                  :amount="market?.vault?.food"
                  :icon-class="vaultResourceIconClass"
                  :text-class="vaultResourceTextClass"
                  class="md:pr-8"
                />
              </div>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>

      <ResourcesGrid v-model="resources" />
    </div>

    <div class="max-sm:w-full sm:max-w-max grid grid-cols-3 items-center justify-center sm:justify-start gap-4">
      <Button
        variant="default"
        :size="sm ? 'default' : 'sm'"
        :disabled="!canBuy"
        @click.stop="buy"
      >
        <span>{{ t("buy") }}</span>
      </Button>
      <Button
        variant="default"
        :size="sm ? 'default' : 'sm'"
        :disabled="!canSell"
        @click.stop="sell"
      >
        <span>{{ t("sell") }}</span>
      </Button>
      <Button
        variant="secondary"
        :size="sm ? 'default' : 'sm'"
        :disabled="resources.isEmpty()"
        @click.stop="clear"
      >
        <span>{{ t("clear") }}</span>
      </Button>
    </div>
  </div>
</template>
