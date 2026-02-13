// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::{Result, anyhow};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHasher, SaltString};
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
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
      .hash_password(self.0.as_bytes(), &salt)
      .map_err(|err| anyhow!("Failed to hash password").context(err))?
      .to_string()
      .into_boxed_str()
      .pipe(Ok)
  }

  pub fn verify(&self, hash: &str) -> bool {
    let result = try {
      let password = self.0.as_bytes();
      let hash = PasswordHash::new(hash)?;
      Argon2::default().verify_password(password, &hash)?
    };

    #[cfg(debug_assertions)]
    if let Err(err) = &result {
      tracing::debug!(error = %err);
    }

    result.is_ok()
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

#[cfg(test)]
mod tests {
  use super::Password;

  #[test]
  fn verify_password() {
    let password = Password::new("foo123");
    let hash = password.hash().unwrap();
    assert!(password.verify(&hash));

    let other_password = Password::new("bar456");
    assert!(!other_password.verify(&hash));
  }
}
