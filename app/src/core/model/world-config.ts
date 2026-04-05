// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type {
  BotAdvancedStartRatio,
  BotDensity,
  Locale,
  WorldConfig,
  WorldId,
  WorldSpeed,
  WorldUnitSpeed,
} from '@/types/core/world';

export class WorldConfigImpl implements WorldConfig {
  public readonly id: WorldId;
  public readonly name: string;
  public readonly locale: Locale;
  public readonly allowCheats: boolean;
  public readonly speed: WorldSpeed;
  public readonly unitSpeed: WorldUnitSpeed;
  public readonly botDensity: BotDensity;
  public readonly botAdvancedStartRatio: BotAdvancedStartRatio;

  private constructor(config: WorldConfig) {
    this.id = config.id;
    this.name = config.name;
    this.locale = config.locale;
    this.allowCheats = config.allowCheats;
    this.speed = config.speed;
    this.unitSpeed = config.unitSpeed;
    this.botDensity = config.botDensity;
    this.botAdvancedStartRatio = config.botAdvancedStartRatio;
  }

  public static create(config: WorldConfig) {
    if (config instanceof WorldConfigImpl) {
      return config;
    }

    return new WorldConfigImpl(config);
  }
}
