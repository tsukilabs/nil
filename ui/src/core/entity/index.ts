// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from './abstract';
import { ChatEntity } from './chat';
import { RoundEntity } from './round';
import { WorldEntity } from './world';
import { PlayerEntity } from './player';
import { VillageEntity } from './village';
import { MilitaryEntity } from './military';

export function initEntities() {
  if (!Object.hasOwn(window, 'NIL')) {
    Object.defineProperty(window, 'NIL', {
      configurable: false,
      enumerable: true,
      writable: false,
      value: {
        update: () => Entity.updateAll(),
      },
    });
  }

  WorldEntity.init();
  ChatEntity.init();
  RoundEntity.init();
  PlayerEntity.init();
  VillageEntity.init();
  MilitaryEntity.init();
}
