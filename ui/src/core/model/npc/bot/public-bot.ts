// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { CoordImpl } from '../../continent/coord';

export class PublicBotImpl implements PublicBot {
  public readonly id: number;
  public readonly name: string;
  public readonly coords: readonly CoordImpl[] = [];

  protected constructor(args: PublicBotImplConstructorArgs) {
    this.id = args.bot.id;
    this.name = args.bot.name;
    this.coords = args.coords.map((it) => CoordImpl.create(it));
  }

  public hasVillage(key: ContinentKey) {
    return this.coords.some((it) => it.is(key));
  }

  public getVillage(key: ContinentKey) {
    return this.coords.find((it) => it.is(key));
  }

  public static create(args: PublicBotImplConstructorArgs) {
    if (args.bot instanceof PublicBotImpl) {
      return args.bot;
    }

    return new PublicBotImpl(args);
  }

  public static async load(id: BotId) {
    const [bot, coords] = await Promise.all([
      commands.getPublicBot(id),
      commands.getBotCoords(id),
    ]);

    return PublicBotImpl.create({ bot, coords });
  }
}

export interface PublicBotImplConstructorArgs {
  bot: PublicBot;
  coords: readonly Coord[];
}
