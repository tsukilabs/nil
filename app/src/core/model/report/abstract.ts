// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import { formatToday, fromZoned } from '@/lib/date';
import type { ComposerTranslation } from 'vue-i18n';
import enUS from '@/locale/en-US/scenes/game/report.json';

export abstract class ReportImpl implements Report_ {
  public readonly id: ReportId;
  public readonly round: RoundId;
  public readonly time: string;
  public readonly date: Date;

  protected constructor(report: Report_) {
    this.id = report.id;
    this.round = report.round;
    this.time = report.time;
    this.date = fromZoned(report.time);
  }

  public abstract getTitle(t: ComposerTranslation<typeof enUS>): string;

  public isUnread() {
    return NIL.report.isUnread(this.id);
  }

  public markRead() {
    NIL.report.markRead(this.id);
  }

  public formatDate() {
    return formatToday(this.date);
  }

  public async goToView() {
    await go('report-view', { params: { id: this.id } });
  }
}
