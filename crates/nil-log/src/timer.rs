// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use jiff::Zoned;
use jiff::fmt::StdFmtWrite;
use jiff::fmt::temporal::DateTimePrinter;
use std::fmt;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

const PRINTER: DateTimePrinter = DateTimePrinter::new()
  .separator(b' ')
  .precision(Some(3));

pub struct Timer;

impl FormatTime for Timer {
  fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result {
    PRINTER
      .print_datetime(&Zoned::now().datetime(), StdFmtWrite(w))
      .map_err(|_| fmt::Error)
  }
}
