// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

/* eslint-disable @typescript-eslint/class-methods-use-this */

import { go } from '@/router';
import { formatDate } from 'date-fns';
import { fromZoned } from '@/lib/date';
import { runWithContext } from '@tb-dev/vue';
import enUS from '@/locale/en-US/scenes/game/report.json';
import ptBR from '@/locale/pt-BR/scenes/game/report.json';
import { type ComposerTranslation, useI18n } from 'vue-i18n';

export abstract class ReportImpl implements Report_ {
  public readonly id: ReportId;
  public readonly timestamp: string;
  public readonly date: Date;

  protected constructor(report: Report_) {
    this.id = report.id;
    this.timestamp = report.timestamp;
    this.date = fromZoned(report.timestamp);
  }

  public abstract getTitle(): string;

  public formatDate() {
    return formatDate(this.date, 'dd/MM HH:mm');
  }

  public async goToView() {
    await go('report-view', { params: { id: this.id } });
  }

  protected i18n(f: (t: ComposerTranslation<typeof enUS>) => string) {
    return runWithContext(() => {
      const { t } = useI18n({
        messages: {
          'en-US': enUS,
          'pt-BR': ptBR,
        },
      });

      return f(t);
    });
  }
}
