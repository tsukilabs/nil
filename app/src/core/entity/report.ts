// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from "./abstract";
import type { Option } from "@tb-dev/utils";
import type { ReportPayload } from "@/types/event";
import { shallowRef, type ShallowRef, triggerRef } from "vue";
import type { ReportId, ReportKind } from "@tsukilabs/nil-bindings";

export class ReportEntity extends Entity {
  private readonly reports = shallowRef<ReportKind[]>([]);
  private readonly unread = shallowRef(new Set<ReportId>());

  public override readonly requireManualUpdates = true;

  constructor() {
    super();
    this.initListeners();
  }

  protected override initListeners() {
    this.event.onReport(this.onReport.bind(this));
  }

  public get(id: ReportId): Option<ReportKind> {
    return this.reports.value.find(({ report }) => report.id === id);
  }

  public has(id: ReportId) {
    return this.reports.value.some(({ report }) => report.id === id);
  }

  public isUnread(id: ReportId) {
    return this.unread.value.has(id);
  }

  public markRead(id: ReportId) {
    if (this.isUnread(id)) {
      this.unread.value.delete(id);
      triggerRef(this.unread);
    }
  }

  private onReport({ report }: ReportPayload) {
    if (!this.has(report.report.id)) {
      this.reports.value.push(report);
      triggerRef(this.reports);

      this.unread.value.add(report.report.id);
      triggerRef(this.unread);
    }
  }

  public remove(id: ReportId) {
    return this.reports.value = this.reports.value.filter(({ report }) => report.id !== id);
  }

  public static use() {
    return super.get(ReportEntity) as ReportEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      reports: instance.reports as Readonly<ShallowRef<readonly ReportKind[]>>,
      unread: instance.unread as Readonly<ShallowRef<ReadonlySet<ReportId>>>,
    } as const;
  }

  public static update() {
    return this.use().update();
  }

  public static getReport(id: ReportId): Option<ReportKind> {
    return this.use().get(id);
  }

  public static getReports(): readonly ReportKind[] {
    return this.use().reports.value;
  }

  public static getUnread(): ReadonlySet<ReportId> {
    return this.use().unread.value;
  }

  public static isUnread(id: ReportId) {
    return this.use().isUnread(id);
  }

  public static markRead(id: ReportId) {
    this.use().markRead(id);
  }

  public static removeReport(id: ReportId) {
    return this.use().remove(id);
  }

  public static init() {
    if (!Object.hasOwn(globalThis.NIL, "report")) {
      const report: (typeof globalThis.NIL)["report"] = {
        getReport: ReportEntity.getReport.bind(ReportEntity),
        getReports: ReportEntity.getReports.bind(ReportEntity),
        getUnread: ReportEntity.getUnread.bind(ReportEntity),
        isUnread: ReportEntity.isUnread.bind(ReportEntity),
        markRead: ReportEntity.markRead.bind(ReportEntity),
        refs: ReportEntity.refs.bind(ReportEntity),
        removeReport: ReportEntity.removeReport.bind(ReportEntity),
        update: ReportEntity.update.bind(ReportEntity),
        use: ReportEntity.use.bind(ReportEntity),
      };

      Object.defineProperty(globalThis.NIL, "report", {
        configurable: false,
        enumerable: true,
        writable: false,
        value: report,
      });
    }
  }
}
