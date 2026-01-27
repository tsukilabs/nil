// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class WorldConfigImpl implements WorldConfig {
  public readonly id: WorldId;
  public readonly name: string;
  public readonly locale: Locale;
  public readonly allowCheats: boolean;

  private constructor(config: WorldConfig) {
    this.id = config.id;
    this.name = config.name;
    this.locale = config.locale;
    this.allowCheats = config.allowCheats;
  }

  public static create(config: WorldConfig) {
    if (config instanceof WorldConfigImpl) {
      return config;
    }

    return new WorldConfigImpl(config);
  }
}
