// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;

impl Client {
  /// GET `/continent/size`
  pub async fn get_continent_size(&self) -> Result<usize> {
    self.http.get_json("continent/size").await
  }
}
