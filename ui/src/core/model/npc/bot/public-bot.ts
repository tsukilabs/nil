// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import * as commands from '@/commands';
import { formatInt } from '@/lib/intl';
import { PLACEHOLDER } from '@/lib/string';
import { CoordImpl } from '@/core/model/continent/coord';
import { RankingEntryImpl } from '@/core/model/ranking/ranking-entry';

export class PublicBotImpl implements PublicBot {
  public readonly id: BotId;
  public readonly coords: readonly CoordImpl[];
  public readonly ranking: Option<RankingEntryImpl>;

  protected constructor(args: PublicBotImplConstructorArgs) {
    this.id = args.bot.id;
    this.coords = args.coords.map((it) => CoordImpl.create(it));

    if (args.ranking) {
      this.ranking = RankingEntryImpl.create(args.ranking);
    }
  }

  public hasCity(key: ContinentKey) {
    return this.coords.some((it) => it.is(key));
  }

  public getCity(key: ContinentKey) {
    return this.coords.find((it) => it.is(key));
  }

  public async goToProfile() {
    await go('profile-bot', { params: { id: this.id } });
  }

  public formatRank() {
    return this.ranking?.rank ? formatInt(this.ranking.rank) : PLACEHOLDER;
  }

  public formatScore() {
    return this.ranking?.score ? formatInt(this.ranking.score) : PLACEHOLDER;
  }

  public static create(args: PublicBotImplConstructorArgs) {
    if (args.bot instanceof PublicBotImpl) {
      return args.bot;
    }

    return new PublicBotImpl(args);
  }

  public static async load(id: BotId) {
    const [bot, coords, ranking] = await Promise.all([
      commands.getPublicBot(id),
      commands.getBotCoords(id),
      commands.getBotRank(id),
    ]);

    return PublicBotImpl.create({ bot, coords, ranking });
  }
}

export interface PublicBotImplConstructorArgs {
  bot: PublicBot;
  coords: readonly Coord[];
  ranking: Option<RankingEntry>;
}
