// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::{Result, anyhow};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHasher, SaltString};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use derive_more::{From, Into};
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use nil_server_types::Password;

#[derive(FromSqlRow, AsExpression, Clone, Debug, From, Into, PartialEq, Eq, Hash)]
#[diesel(sql_type = Text)]
pub struct HashedPassword(String);

impl HashedPassword {
  pub fn new(password: &Password) -> Result<Self> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
      .hash_password(password.as_bytes(), &salt)
      .map_err(|_| anyhow!("Failed to hash password"))?
      .to_string();

    Ok(Self(hash))
  }

  pub fn verify(&self, password: &Password) -> bool {
    let result = try {
      let password = password.as_bytes();
      let hash = PasswordHash::new(&self.0)?;
      Argon2::default().verify_password(password, &hash)?
    };

    result.is_ok()
  }
}

impl FromSql<Text, Sqlite> for HashedPassword {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    Ok(HashedPassword(value))
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

#[cfg(test)]
mod tests {
  use super::HashedPassword;
  use nil_server_types::Password;

  #[test]
  fn verify_password() {
    let password = Password::new("foo123");
    let hashed = HashedPassword::new(&password).unwrap();
    assert!(hashed.verify(&password));

    let other_password = Password::new("bar456");
    assert!(!hashed.verify(&other_password));
  }
}
