// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::{Deref, From, Into};
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use jiff::SignedDuration;
use nil_server_types::round::RoundDuration;
use nil_server_types::time::Minutes;
use serde::{Deserialize, Serialize};
use std::time::Duration as StdDuration;

#[derive(
  FromSqlRow, AsExpression, Clone, Copy, Debug, Deref, From, Into, Deserialize, Serialize,
)]
#[diesel(sql_type = Text)]
pub struct Duration(StdDuration);

impl FromSql<Text, Sqlite> for Duration {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    Ok(Duration(StdDuration::from_millis(value.parse()?)))
  }
}

impl ToSql<Text, Sqlite> for Duration
where
  String: ToSql<Text, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    out.set_value(self.0.as_millis().to_string());
    Ok(IsNull::No)
  }
}

impl From<Minutes> for Duration {
  fn from(mins: Minutes) -> Self {
    Self(StdDuration::from(mins))
  }
}

impl From<RoundDuration> for Duration {
  fn from(duration: RoundDuration) -> Self {
    Self(StdDuration::from(duration))
  }
}

impl From<Duration> for RoundDuration {
  fn from(duration: Duration) -> Self {
    RoundDuration::from(duration.0)
  }
}

impl TryFrom<SignedDuration> for Duration {
  type Error = crate::error::Error;

  fn try_from(duration: SignedDuration) -> Result<Self, Self::Error> {
    Ok(Duration(StdDuration::try_from(duration)?))
  }
}

impl TryFrom<Duration> for SignedDuration {
  type Error = crate::error::Error;

  fn try_from(duration: Duration) -> Result<Self, Self::Error> {
    Ok(SignedDuration::try_from(duration.0)?)
  }
}
