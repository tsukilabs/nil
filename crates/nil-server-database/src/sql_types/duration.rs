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
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(FromSqlRow, AsExpression, Clone, Debug, Deref, From, Into, Deserialize, Serialize)]
#[diesel(sql_type = Text)]
pub struct SqlDuration(SignedDuration);

impl FromSql<Text, Sqlite> for SqlDuration {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    Ok(SqlDuration(SignedDuration::from_str(value.as_str())?))
  }
}

impl ToSql<Text, Sqlite> for SqlDuration
where
  String: ToSql<Text, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    out.set_value(self.to_string());
    Ok(IsNull::No)
  }
}
