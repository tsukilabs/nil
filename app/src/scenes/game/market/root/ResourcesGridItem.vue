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
import type { MarketFee, Resources } from "@tsukilabs/nil-bindings";
import { NumberField, NumberFieldContent, NumberFieldInput } from "@ui/number-field";

const props = defineProps<{
  kind: keyof Resources;
  marketFee?: Option<MarketFee>;
  limitToAvailable?: boolean;
}>();

const amount = defineModel<number>({ required: true });

const { t } = useI18n();

const { player } = NIL.player.refs();
const available = computed(() => {
  const value = player.value?.resources[props.kind] ?? 0;
  if (
    typeof props.marketFee === "number" &&
    Number.isFinite(props.marketFee) &&
    props.marketFee >= CONSTS.marketFeeMin &&
    props.marketFee <= CONSTS.marketFeeMax
  ) {
    return Math.ceil(Math.max(0, value - (value * props.marketFee)));
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
            v-if="limitToAvailable"
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
