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
use std::time::Duration;

#[derive(
  FromSqlRow, AsExpression, Clone, Copy, Debug, Deref, From, Into, Deserialize, Serialize,
)]
#[diesel(sql_type = Text)]
pub struct SqlDuration(Duration);

impl FromSql<Text, Sqlite> for SqlDuration {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    Ok(SqlDuration(Duration::from_millis(value.parse()?)))
  }
}

impl ToSql<Text, Sqlite> for SqlDuration
where
  String: ToSql<Text, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    out.set_value(self.0.as_millis().to_string());
    Ok(IsNull::No)
  }
}

impl From<Minutes> for SqlDuration {
  fn from(mins: Minutes) -> Self {
    Self(Duration::from(mins))
  }
}

impl From<RoundDuration> for SqlDuration {
  fn from(duration: RoundDuration) -> Self {
    Self(Duration::from(duration))
  }
}

impl From<SqlDuration> for RoundDuration {
  fn from(duration: SqlDuration) -> Self {
    RoundDuration::from(duration.0)
  }
}

impl TryFrom<SignedDuration> for SqlDuration {
  type Error = crate::error::Error;

  fn try_from(duration: SignedDuration) -> Result<Self, Self::Error> {
    Ok(SqlDuration(Duration::try_from(duration)?))
  }
}

impl TryFrom<SqlDuration> for SignedDuration {
  type Error = crate::error::Error;

  fn try_from(duration: SqlDuration) -> Result<Self, Self::Error> {
    Ok(SignedDuration::try_from(duration.0)?)
  }
}
