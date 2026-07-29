// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use std::env;

const LICENSE: &str = "
// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only";

fn main() -> Result<()> {
  if nil_env::generate_ffi_bindings() {
    generate_node()?;
    generate_c()?;
    generate_csharp();
  }

  Ok(())
}

fn generate_node() -> Result<()> {
  nil_ffi_node::generate()
    .input("src/lib.rs")
    .output("../../packages/ffi/src/def.ts")
    .call()?;

  Ok(())
}

fn generate_c() -> Result<()> {
  cbindgen::Builder::new()
    .with_crate(env::var("CARGO_MANIFEST_DIR")?)
    .with_language(cbindgen::Language::C)
    .with_header(LICENSE.trim_start())
    .generate()?
    .write_to_file("gen/libnil.h");

  Ok(())
}

fn generate_csharp() {
  let license = LICENSE.trim_start();
  csbindgen::Builder::default()
    .input_extern_file("src/lib.rs")
    .input_extern_file("src/status.rs")
    .csharp_dll_name("libnil")
    .csharp_class_name("libnil")
    .csharp_class_accessibility("public")
    .csharp_type_rename(rename_csharp_type)
    .csharp_file_header(format!("{license}\n"))
    .generate_csharp_file("gen/libnil.cs")
    .unwrap();
}

fn rename_csharp_type(name: String) -> String {
  match name.as_str() {
    "RequestId" => "uint".into(),
    "Status" => "StatusCode".into(),
    _ => name,
  }
}
