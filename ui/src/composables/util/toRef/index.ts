// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { type Ref, toRef } from 'vue';
import type { CityImpl } from '@/core/model/city/city';
import type { PlayerImpl } from '@/core/model/player/player';
import type { CoordImpl } from '@/core/model/continent/coord';

export function toCoordRef(coord?: MaybeNilRef<CoordImpl>) {
  return (coord ? toRef(coord) : NIL.city.refs().coord) as Ref<Option<CoordImpl>>;
}

export function toContinentKeyRef(key?: MaybeNilRef<ContinentKey>) {
  return (key ? toRef(key) : NIL.city.refs().coord) as Ref<Option<ContinentKey>>;
}

export function toPlayerRef(player?: MaybeNilRef<PlayerImpl>) {
  return (player ? toRef(player) : NIL.player.refs().player) as Ref<Option<PlayerImpl>>;
}

export function toCityRef(city?: MaybeNilRef<CityImpl>) {
  return (city ? toRef(city) : NIL.city.refs().city) as Ref<Option<CityImpl>>;
}
