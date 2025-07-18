// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::http::{json, request};
use anyhow::anyhow;
use futures::TryFutureExt;
use markdown::to_html_with_options;
use reqwest::{Method, Url};
use semver::Version;
use serde::{Deserialize, Serialize};

pub const NSR: &str = "https://nil.dev.br/nsr";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NsrScript {
  id: String,
  readme: Url,
  script: Url,
  frontmatter: NsrScriptFrontmatter,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NsrScriptFrontmatter {
  name: String,
  description: String,
  author: String,
  version: Version,
  official: bool,
  readonly: bool,
}

pub async fn registry() -> Result<Vec<NsrScript>> {
  let url = format!("{NSR}/registry.json");
  request(Method::GET, &url)
    .call()
    .and_then(async |res| json(res).await)
    .await
}

pub async fn script(id: &str) -> Result<String> {
  let url = format!("{NSR}/{id}/script.lua");
  request(Method::GET, &url)
    .call()
    .await?
    .text()
    .await
    .map_err(Into::into)
}

pub async fn readme(id: &str) -> Result<String> {
  let url = format!("{NSR}/{id}/readme.md");
  request(Method::GET, &url)
    .call()
    .await?
    .text()
    .await
    .map(|md| parse_markdown(&md))?
}

fn parse_markdown(md: &str) -> Result<String> {
  let options = markdown::Options {
    compile: markdown::CompileOptions::default(),
    parse: markdown::ParseOptions {
      constructs: markdown::Constructs {
        frontmatter: true,
        ..Default::default()
      },
      ..Default::default()
    },
  };

  to_html_with_options(md, &options)
    .map_err(|_| anyhow!("Failed to parse script markdown"))
    .map_err(Into::into)
}
