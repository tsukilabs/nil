// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from './abstract';
import { asyncRef } from '@tb-dev/vue';
import { getPlayerReports } from '@/commands';
import { shallowRef, type ShallowRef, triggerRef } from 'vue';

export class ReportEntity extends Entity {
  private readonly reports: ShallowRef<ReportId[]>;
  private readonly unread = shallowRef(new Set<ReportId>());

  public readonly updateReports: () => Promise<void>;

  public override readonly requireManualUpdates = true;

  constructor() {
    super();

    const reports = asyncRef([], getPlayerReports);
    this.reports = reports.state;
    this.updateReports = reports.execute;

    this.initListeners();
  }

  protected override initListeners() {
    this.event.onReport(this.onReport.bind(this));
  }

  public override async update() {
    await this.updateReports();
  }

  private onReport({ report }: ReportPayload) {
    if (!this.reports.value.includes(report)) {
      this.reports.value.push(report);
      triggerRef(this.reports);

      this.unread.value.add(report);
      triggerRef(this.unread);
    }
  }

  public static use() {
    return super.get(ReportEntity) as ReportEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      reports: instance.reports as Readonly<ShallowRef<readonly ReportId[]>>,
      unread: instance.unread as Readonly<ShallowRef<ReadonlySet<ReportId>>>,
    } as const;
  }

  public static update() {
    return this.use().update();
  }

  public static getReports(): readonly ReportId[] {
    return this.use().reports.value;
  }

  public static getUnread(): ReadonlySet<ReportId> {
    return this.use().unread.value;
  }

  public static isUnread(id: ReportId) {
    return this.getUnread().has(id);
  }

  public static init() {
    if (!Object.hasOwn(globalThis.NIL, 'report')) {
      const report: (typeof globalThis.NIL)['report'] = {
        getReports: ReportEntity.getReports.bind(ReportEntity),
        getUnread: ReportEntity.getUnread.bind(ReportEntity),
        isUnread: ReportEntity.isUnread.bind(ReportEntity),
        refs: ReportEntity.refs.bind(ReportEntity),
        update: ReportEntity.update.bind(ReportEntity),
        use: ReportEntity.use.bind(ReportEntity),
      };

      Object.defineProperty(globalThis.NIL, 'report', {
        configurable: false,
        enumerable: true,
        writable: false,
        value: report,
      });
    }
  }
}
