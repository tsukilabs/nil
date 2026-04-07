// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::{Deref, Display, From, Into};
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use nil_core::player::PlayerId as CorePlayerId;

#[derive(
  FromSqlRow, AsExpression, Clone, Debug, Deref, Display, From, Into, PartialEq, Eq, Hash,
)]
#[diesel(sql_type = Text)]
pub struct PlayerId(CorePlayerId);

impl FromSql<Text, Sqlite> for PlayerId {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    Ok(PlayerId(CorePlayerId::from(value)))
  }
}

impl ToSql<Text, Sqlite> for PlayerId
where
  String: ToSql<Text, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    out.set_value(self.0.as_str());
    Ok(IsNull::No)
  }
}

impl From<&CorePlayerId> for PlayerId {
  fn from(id: &CorePlayerId) -> Self {
    Self(id.clone())
  }
}
