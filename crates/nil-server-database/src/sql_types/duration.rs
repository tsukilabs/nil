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
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(
  FromSqlRow, AsExpression, Clone, Copy, Debug, Deref, From, Into, Deserialize, Serialize,
)]
#[diesel(sql_type = Text)]
pub struct db_Duration(Duration);

impl FromSql<Text, Sqlite> for db_Duration {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    Ok(db_Duration(Duration::from_millis(value.parse()?)))
  }
}

impl ToSql<Text, Sqlite> for db_Duration
where
  String: ToSql<Text, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    out.set_value(self.0.as_millis().to_string());
    Ok(IsNull::No)
  }
}

impl const From<RoundDuration> for db_Duration {
  fn from(duration: RoundDuration) -> Self {
    Self(Duration::from(duration))
  }
}

impl const From<db_Duration> for RoundDuration {
  fn from(value: db_Duration) -> Self {
    RoundDuration::from(value.0)
  }
}

impl TryFrom<SignedDuration> for db_Duration {
  type Error = crate::error::Error;

  fn try_from(duration: SignedDuration) -> Result<Self, Self::Error> {
    Ok(db_Duration(Duration::try_from(duration)?))
  }
}

impl TryFrom<db_Duration> for SignedDuration {
  type Error = crate::error::Error;

  fn try_from(duration: db_Duration) -> Result<Self, Self::Error> {
    Ok(SignedDuration::try_from(duration.0)?)
  }
}
