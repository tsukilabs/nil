<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import { computed } from "vue";
import { Label } from "@ui/label";
import { useI18n } from "vue-i18n";
import { CONSTS } from "@/lib/global";
import { formatInt } from "@/lib/intl";
import type { Option } from "@tb-dev/utils";
import Food from "@/components/resources/Food.vue";
import Iron from "@/components/resources/Iron.vue";
import Wood from "@/components/resources/Wood.vue";
import Stone from "@/components/resources/Stone.vue";
import type { Resources } from "@tsukilabs/nil-bindings";
import type { MarketImpl } from "@/core/model/market/market";
import { NumberField, NumberFieldContent, NumberFieldInput } from "@ui/number-field";

const props = defineProps<{
  kind: keyof Resources;
  market?: Option<MarketImpl>;
  limitToAvailable?: boolean;
}>();

const amount = defineModel<number>({ required: true });

const { t } = useI18n();

const { player } = NIL.player.refs();
const available = computed(() => {
  const value = player.value?.resources[props.kind] ?? 0;
  if (
    typeof props.market?.fee === "number" &&
    Number.isFinite(props.market.fee) &&
    props.market.fee >= CONSTS.marketFeeMin &&
    props.market.fee <= CONSTS.marketFeeMax
  ) {
    return Math.ceil(Math.max(0, value - (value * props.market.fee)));
  }
  else {
    return value;
  }
});

function toggleMax() {
  if (amount.value !== available.value) {
    amount.value = available.value;
  }
  else {
    amount.value = 0;
  }
}
</script>

<template>
  <div>
    <Label>
      <div class="flex justify-start items-center gap-1.5 text-xs 2xl:text-sm text-muted-foreground">
        <Food v-if="kind === 'food'" hide-amount />
        <Iron v-else-if="kind === 'iron'" hide-amount />
        <Stone v-else-if="kind === 'stone'" hide-amount />
        <Wood v-else-if="kind === 'wood'" hide-amount />
        <div>
          <span>{{ t(kind) }}</span>
          <span
            v-if="market && limitToAvailable"
            class="cursor-pointer"
            @click.stop="toggleMax"
          >
            {{ ` (${formatInt(available)})` }}
          </span>
        </div>
      </div>
      <NumberField
        v-model="amount"
        :disabled="!player || (limitToAvailable && available <= 0)"
        :min="0"
        :max="limitToAvailable ? available : undefined"
        :step="1"
        :default-value="0"
        invert-wheel-change
        class="w-full"
      >
        <NumberFieldContent>
          <NumberFieldInput class="dark:bg-input/40 max-sm:h-6 max-sm:text-xs" />
        </NumberFieldContent>
      </NumberField>
    </Label>
  </div>
</template>
