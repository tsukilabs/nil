// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::{From, Into};
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use nil_core::world::WorldId;

#[derive(FromSqlRow, AsExpression, Clone, Debug, From, Into, PartialEq, Eq, Hash)]
#[diesel(sql_type = Text)]
pub struct GameId(WorldId);

impl FromSql<Text, Sqlite> for GameId {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    Ok(GameId(WorldId::try_from(value.as_str())?))
  }
}

impl ToSql<Text, Sqlite> for GameId
where
  String: ToSql<Text, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    out.set_value(self.0.to_string());
    Ok(IsNull::No)
  }
}
