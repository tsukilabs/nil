// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_core::report::ReportKind;
use nil_payload::report::*;

impl Client {
  pub async fn get_report(&self, req: GetReportRequest) -> Result<ReportKind> {
    http::json_post("get-report")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn get_reports(&self, req: GetReportsRequest) -> Result<Vec<ReportKind>> {
    http::json_post("get-reports")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }
}
