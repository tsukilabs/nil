// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import { getRank } from '@/commands';
import { formatInt } from '@/lib/intl';

export class RankingEntryImpl implements RankingEntry {
  public readonly rank: number;
  public readonly ruler: Ruler;
  public readonly score: number;
  public readonly cities: number;

  private constructor(entry: RankingEntry) {
    this.rank = entry.rank;
    this.ruler = entry.ruler;
    this.score = entry.score;
    this.cities = entry.cities;
  }

  public async goToProfile() {
    const id = this.ruler.id;
    await go(`profile-${this.ruler.kind}`, { params: { id } });
  }

  public formatRank() {
    return formatInt(this.rank);
  }

  public formatScore() {
    return formatInt(this.score);
  }

  public formatCities() {
    return formatInt(this.cities);
  }

  get uniqueId() {
    const { id, kind } = this.ruler;
    return `${kind}-${id}`;
  }

  public static create(entry: RankingEntry) {
    if (entry instanceof RankingEntryImpl) {
      return entry;
    }

    return new RankingEntryImpl(entry);
  }

  public static async load(id: Ruler) {
    const entry = await getRank(id);
    return entry ? RankingEntryImpl.create(entry) : null;
  }
}
