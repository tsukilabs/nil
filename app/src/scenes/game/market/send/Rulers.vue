<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import { computed } from "vue";
import type { Option } from "@tb-dev/utils";
import { useI18n } from "@tsukilabs/nil-i18n";
import type { Ruler } from "@tsukilabs/nil-bindings";
import { useRulersByKind } from "@/composables/ruler/useRulersByKind";
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from "@ui/select";

const props = defineProps<{
  rulers: Ruler[];
  loading: boolean;
}>();

const recipient = defineModel<Option<Ruler>>({ required: true });

const { t } = useI18n();

const { bots, players, precursors } = useRulersByKind(() => props.rulers, {
  allowCurrentPlayer: false,
});

const rulerId = computed({
  get: () => {
    if (recipient.value) {
      return `${recipient.value.kind}|${recipient.value.id}`;
    }
    else {
      return null;
    }
  },
  set: (id) => {
    if (id) {
      const parts = id.split("|", 2);
      const ruler = props.rulers.find((it) => {
        return it.kind === parts[0] && it.id === parts[1];
      });

      recipient.value = ruler ?? null;
    }
    else {
      recipient.value = null;
    }
  },
});
</script>

<template>
  <div>
    <Select v-model="rulerId" :disabled="loading || rulers.length === 0">
      <SelectTrigger class="w-full">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup v-if="players.length > 0">
          <SelectLabel>{{ t("player", 2) }}</SelectLabel>
          <SelectItem
            v-for="player of players"
            :key="player.id"
            :value="`player|${player.id}`"
          >
            {{ player.id }}
          </SelectItem>
        </SelectGroup>
        <SelectGroup v-if="bots.length > 0">
          <SelectLabel>{{ t("bot", 2) }}</SelectLabel>
          <SelectItem
            v-for="bot of bots"
            :key="bot.id"
            :value="`bot|${bot.id}`"
          >
            {{ bot.id }}
          </SelectItem>
        </SelectGroup>
        <SelectGroup v-if="precursors.length > 0">
          <SelectLabel>{{ t("precursor", 2) }}</SelectLabel>
          <SelectItem
            v-for="precursor of precursors"
            :key="precursor.id"
            :value="`precursor|${precursor.id}`"
          >
            {{ precursor.id }}
          </SelectItem>
        </SelectGroup>
      </SelectContent>
    </Select>
  </div>
</template>
