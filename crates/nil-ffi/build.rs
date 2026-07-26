// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;

fn main() -> Result<()> {
  nil_ffi_node::generate()
    .input("./src/lib.rs")
    .output("../../packages/ffi/src/def.ts")
    .call()?;

  Ok(())
}
