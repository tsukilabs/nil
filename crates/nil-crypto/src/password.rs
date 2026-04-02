// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use anyhow::anyhow;
use argon2::password_hash::{Error as PasswordHashError, PasswordHasher};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;
use tap::Pipe;

#[derive(Clone, Default, From, Into, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[from(String, &str, Arc<str>, Box<str>, Cow<'_, str>)]
pub struct Password(Arc<str>);

impl Password {
  #[inline]
  pub fn new(password: &str) -> Self {
    Self(Arc::from(password))
  }

  pub fn hash(&self) -> Result<Box<str>> {
    argon2()
      .hash_password(self.0.as_bytes())
      .map_err(|err| anyhow!("Failed to hash password").context(err))?
      .to_string()
      .into_boxed_str()
      .pipe(Ok)
  }

  pub fn verify(&self, hash: &str) -> Result<bool> {
    match verify(self.0.as_bytes(), hash) {
      Ok(()) => Ok(true),
      Err(PasswordHashError::PasswordInvalid) => Ok(false),
      Err(err) => {
        #[cfg(debug_assertions)]
        tracing::trace!(error = %err);

        Err(err.into())
      }
    }
  }
}

impl Deref for Password {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    self.0.as_str()
  }
}

impl fmt::Debug for Password {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("Password")
      .field(&"***")
      .finish()
  }
}

fn argon2() -> Argon2<'static> {
  Argon2::default()
}

fn verify(password: &[u8], hash: &str) -> Result<(), PasswordHashError> {
  let hash = PasswordHash::new(hash)?;
  argon2().verify_password(password, &hash)
}

#[cfg(test)]
mod tests {
  use super::Password;
  use anyhow::Result;

  #[test]
  fn verify_password() -> Result<()> {
    let password = Password::new("foo123");
    let hash = password.hash().unwrap();
    assert!(password.verify(&hash)?);

    let other_password = Password::new("bar456");
    assert!(!other_password.verify(&hash)?);

    Ok(())
  }
}
