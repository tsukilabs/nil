// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { shallowRef, toRef } from 'vue';
import { watchImmediate } from '@vueuse/core';
import { PublicBotImpl } from '@/core/model/npc/bot/public-bot';
import { PublicPlayerImpl } from '@/core/model/player/public-player';
import { PublicPrecursorImpl } from '@/core/model/npc/precursor/public-precursor';

export function usePublicCityOwner(owner: MaybeNilRef<Ruler>) {
  const bot = shallowRef<Option<PublicBotImpl>>();
  const player = shallowRef<Option<PublicPlayerImpl>>();
  const precursor = shallowRef<Option<PublicPrecursorImpl>>();

  watchImmediate(toRef(owner), load);

  async function load(value?: Option<Ruler>) {
    reset(value?.kind);

    if (value?.kind) {
      switch (value.kind) {
        case 'bot': {
          bot.value = await PublicBotImpl.load(value.id);
          break;
        }
        case 'player': {
          player.value = await PublicPlayerImpl.load(value.id);
          break;
        }
        case 'precursor': {
          precursor.value = await PublicPrecursorImpl.load(value.id);
          break;
        }
      }
    }
  }

  function reset(ignore?: Option<RulerKind>) {
    switch (ignore ?? null) {
      case 'bot': {
        player.value = null;
        precursor.value = null;
        break;
      }
      case 'player': {
        bot.value = null;
        precursor.value = null;
        break;
      }
      case 'precursor': {
        bot.value = null;
        player.value = null;
        break;
      }
      default: {
        bot.value = null;
        player.value = null;
        precursor.value = null;
      }
    }
  }

  return {
    bot: bot as Readonly<typeof bot>,
    player: player as Readonly<typeof player>,
    precursor: precursor as Readonly<typeof precursor>,
    load,
    reset,
  };
}
