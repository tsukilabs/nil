// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use derive_more::{From, Into};
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use nil_crypto::password::Password;
use std::fmt;

#[derive(FromSqlRow, AsExpression, Clone, From, Into, PartialEq, Eq, Hash)]
#[diesel(sql_type = Text)]
pub struct HashedPassword(Box<str>);

impl HashedPassword {
  #[inline]
  pub fn new(password: &Password) -> Result<Self> {
    Ok(Self(password.hash()?))
  }

  #[inline]
  pub fn verify(&self, password: &Password) -> bool {
    password.verify(&self.0)
  }
}

impl FromSql<Text, Sqlite> for HashedPassword {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    Ok(HashedPassword(Box::from(value)))
  }
}

impl ToSql<Text, Sqlite> for HashedPassword
where
  String: ToSql<Text, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    out.set_value(self.0.as_str());
    Ok(IsNull::No)
  }
}

impl fmt::Debug for HashedPassword {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("HashedPassword")
      .field(&"***")
      .finish()
  }
}
