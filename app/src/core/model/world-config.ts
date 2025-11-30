// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class WorldConfigImpl implements WorldConfig {
  public readonly name: string;
  public readonly allowCheats: boolean;

  private constructor(args: WorldConfig) {
    this.name = args.name;
    this.allowCheats = args.allowCheats;
  }

  public static create(config: WorldConfig) {
    return new WorldConfigImpl(config);
  }
}
