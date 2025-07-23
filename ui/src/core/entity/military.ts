// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from './abstract';

export class MilitaryEntity extends Entity {
  constructor() {
    super();

    this.initListeners();
  }

  public static use() {
    return super.get(MilitaryEntity) as MilitaryEntity;
  }

  public static update() {
    return this.use().update();
  }

  public static init() {
    if (!Object.hasOwn(window.NIL, 'military')) {
      const military: (typeof window.NIL)['military'] = {};

      Object.defineProperty(window.NIL, 'military', {
        configurable: false,
        enumerable: true,
        writable: false,
        value: military,
      });
    }
  }
}
