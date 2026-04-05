// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface WorldOptions {
  readonly name: string;
  readonly size: number;
  readonly locale: Locale;
  readonly allowCheats: boolean;
  readonly speed: WorldSpeed;
  readonly unitSpeed: WorldUnitSpeed;
  readonly botDensity: BotDensity;
  readonly botAdvancedStartRatio: BotAdvancedStartRatio;
}

export interface WorldConfig {
  readonly id: WorldId;
  readonly name: string;
  readonly locale: Locale;
  readonly allowCheats: boolean;
  readonly speed: WorldSpeed;
  readonly unitSpeed: WorldUnitSpeed;
  readonly botDensity: BotDensity;
  readonly botAdvancedStartRatio: BotAdvancedStartRatio;
}

export type WorldId = string;

export type Locale = 'en-US' | 'pt-BR';

export type WorldSpeed = number;

export type WorldUnitSpeed = number;

export type BotDensity = number;

export type BotAdvancedStartRatio = number;

export interface WorldStats {
  readonly infrastructure: InfrastructureStats;
}
