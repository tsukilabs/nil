// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from './abstract';
import { ChatEntity } from './chat';
import { CityEntity } from './city';
import { RoundEntity } from './round';
import { WorldEntity } from './world';
import { PlayerEntity } from './player';
import { MilitaryEntity } from './military';

export function initEntities() {
  if (!Object.hasOwn(globalThis, 'NIL')) {
    Object.defineProperty(globalThis, 'NIL', {
      configurable: false,
      enumerable: true,
      writable: false,
      value: {
        update: () => Entity.updateAll(),
      },
    });
  }

  Entity.dispose();

  WorldEntity.init();
  ChatEntity.init();
  RoundEntity.init();
  PlayerEntity.init();
  CityEntity.init();
  MilitaryEntity.init();
}
