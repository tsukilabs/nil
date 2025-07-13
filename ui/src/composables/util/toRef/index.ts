// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { type Ref, toRef } from 'vue';
import type { Option } from '@tb-dev/utils';
import type { MaybeNilRef } from '@tb-dev/vue';
import type { CoordImpl } from '@/core/model/coord';
import type { PlayerImpl } from '@/core/model/player';
import type { VillageImpl } from '@/core/model/village';

export function toCoordRef(coord?: MaybeNilRef<CoordImpl>) {
  return (coord ? toRef(coord) : NIL.village.refs().coord) as Ref<Option<CoordImpl>>;
}

export function toPlayerRef(player?: MaybeNilRef<PlayerImpl>) {
  return (player ? toRef(player) : NIL.player.refs().player) as Ref<Option<PlayerImpl>>;
}

export function toVillageRef(village?: MaybeNilRef<VillageImpl>) {
  return (village ? toRef(village) : NIL.village.refs().village) as Ref<Option<VillageImpl>>;
}
