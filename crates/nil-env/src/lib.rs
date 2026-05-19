// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(const_clone, const_cmp, derive_const)]

use anyhow::{Error, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::env;
use std::ffi::OsStr;
use std::num::NonZeroU16;
use std::path::Path;
use strum::{AsRefStr, Display, EnumString};
use url::Url;

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Copy, Debug, Display, AsRefStr, EnumString, Deserialize, Serialize)]
#[derive_const(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, repr(enum = name)))]
#[cfg_attr(feature = "typescript", ts(rename = "env_Var"))]
pub enum Var {
  #[serde(rename = "NIL_DATABASE_URL")]
  #[strum(serialize = "NIL_DATABASE_URL")]
  DatabaseUrl,

  #[serde(rename = "NIL_JWT_SECRET")]
  #[strum(serialize = "NIL_JWT_SECRET")]
  JwtSecret,

  #[serde(rename = "NIL_LOG_DIR")]
  #[strum(serialize = "NIL_LOG_DIR")]
  LogDir,

  #[serde(rename = "NIL_LOG_LEVEL")]
  #[strum(serialize = "NIL_LOG_LEVEL")]
  LogLevel,

  #[serde(rename = "NIL_LOG_TOWER_HTTP")]
  #[strum(serialize = "NIL_LOG_TOWER_HTTP")]
  LogTowerHttp,

  #[serde(rename = "NIL_MINIFY_SOURCE")]
  #[strum(serialize = "NIL_MINIFY_SOURCE")]
  MinifySource,

  #[serde(rename = "NIL_REMOTE_SERVER_ADDR")]
  #[strum(serialize = "NIL_REMOTE_SERVER_ADDR")]
  RemoteServerAddr,

  #[serde(rename = "NIL_REMOTE_WORLD_LIMIT")]
  #[strum(serialize = "NIL_REMOTE_WORLD_LIMIT")]
  RemoteWorldLimit,

  #[serde(rename = "NIL_REMOTE_WORLD_LIMIT_PER_USER")]
  #[strum(serialize = "NIL_REMOTE_WORLD_LIMIT_PER_USER")]
  RemoteWorldLimitPerUser,
}

impl Var {
  /// # Safety
  ///
  /// See [`std::env::set_var`].
  pub unsafe fn set(self, value: impl AsRef<OsStr>) {
    unsafe { env::set_var(self, value) }
  }

  /// # Safety
  ///
  /// See [`std::env::remove_var`].
  pub unsafe fn remove(self) {
    unsafe { env::remove_var(self) }
  }
}

impl AsRef<OsStr> for Var {
  fn as_ref(&self) -> &OsStr {
    OsStr::new(<Var as AsRef<str>>::as_ref(self))
  }
}

pub fn database_url() -> Result<Box<str>> {
  env::var(Var::DatabaseUrl)
    .map(String::into_boxed_str)
    .map_err(|_| on_var_err(Var::DatabaseUrl))
}

pub fn jwt_secret() -> Box<str> {
  // Using a known secret should not be a problem for local servers.
  env::var(Var::JwtSecret)
    .map(String::into_boxed_str)
    .unwrap_or_else(|_| Box::from("CALL-OF-NIL"))
}

pub fn log_dir() -> Result<Box<Path>> {
  env::var(Var::LogDir)
    .map(|dir| Box::from(Path::new(&dir)))
    .map_err(|_| on_var_err(Var::LogDir))
}

pub fn log_level() -> Box<str> {
  env::var(Var::LogLevel)
    .map(String::into_boxed_str)
    .unwrap_or_else(|_| Box::from("trace"))
}

pub fn log_tower_http() -> bool {
  env::var(Var::LogTowerHttp).is_ok_and(|it| it == "true")
}

pub fn remote_server_addr() -> Url {
  if let Ok(addr) = env::var(Var::RemoteServerAddr)
    && let Ok(url) = Url::parse(&addr)
  {
    url
  } else {
    Url::parse("https://tsukilabs.dev.br/nil/").unwrap()
  }
}

pub fn remote_world_limit() -> NonZeroU16 {
  env::var(Var::RemoteWorldLimit)
    .ok()
    .and_then(|it| it.parse::<NonZeroU16>().ok())
    .unwrap_or_else(|| unsafe { NonZeroU16::new_unchecked(100) })
}

pub fn remote_world_limit_per_user() -> NonZeroU16 {
  env::var(Var::RemoteWorldLimitPerUser)
    .ok()
    .and_then(|it| it.parse::<NonZeroU16>().ok())
    .unwrap_or_else(|| unsafe { NonZeroU16::new_unchecked(3) })
}

pub(crate) fn on_var_err(var: Var) -> Error {
  anyhow!("environment variable not found: {var}")
}
