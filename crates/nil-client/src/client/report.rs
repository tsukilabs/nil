// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::report::ReportKind;
use nil_payload::report::{GetReportRequest, GetReportsRequest};

impl Client {
  pub async fn get_report(&self, req: GetReportRequest) -> Result<ReportKind> {
    self.http.json_post("get-report", req).await
  }

  pub async fn get_reports(&self, req: GetReportsRequest) -> Result<Vec<ReportKind>> {
    self.http.json_post("get-reports", req).await
  }
}
