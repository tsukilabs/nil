// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export abstract class ReportImpl implements Report_ {
  public readonly id: ReportId;
  public readonly timestamp: string;

  protected constructor(report: Report_) {
    this.id = report.id;
    this.timestamp = report.timestamp;
  }
}
