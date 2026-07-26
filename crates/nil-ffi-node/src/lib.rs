// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![feature(iterator_try_collect)]

use anyhow::{Result, bail};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use syn::{Abi, FnArg, Item, ItemFn, ReturnType, Type, Visibility};

#[bon::builder]
pub fn generate(input: impl AsRef<Path>, output: impl AsRef<Path>) -> Result<()> {
  let content = fs::read_to_string(input)?;
  let file = syn::parse_file(&content)?;
  let functions = collect_functions(&file);

  let mut result = with_header()?;
  writeln!(&mut result, "export const definitions = {{")?;

  for function in functions {
    let name = function.sig.ident.to_string();
    let arguments = function
      .sig
      .inputs
      .iter()
      .map(|arg| {
        match arg {
          FnArg::Receiver(_) => bail!("FFI function cannot have self receiver"),
          FnArg::Typed(arg) => node_type(&arg.ty),
        }
      })
      .try_collect::<Vec<_>>()?;

    let return_type = match &function.sig.output {
      ReturnType::Default => "ffi.types.VOID",
      ReturnType::Type(_, ty) => node_type(ty)?,
    };

    writeln!(
      &mut result,
      "{name}: {{ arguments: [{}], return: {return_type} }},",
      arguments.join(", ")
    )?;
  }

  writeln!(&mut result, "}} as const;")?;

  fs::write(output, result)?;

  Ok(())
}

#[rustfmt::skip]
fn with_header() -> Result<String> {
  let mut buf = String::new();
  writeln!(buf, "// dprint-ignore-file\n")?;
  writeln!(buf, "// Copyright (C) Call of Nil contributors")?;
  writeln!(buf, "// SPDX-License-Identifier: AGPL-3.0-only\n")?;
  writeln!(buf, "import * as ffi from \"node:ffi\";\n")?;
  Ok(buf)
}

fn collect_functions(file: &syn::File) -> Vec<&ItemFn> {
  file
    .items
    .iter()
    .filter_map(|item| {
      if let Item::Fn(function) = item {
        is_ffi_function(function).then_some(function)
      } else {
        None
      }
    })
    .collect()
}

fn is_ffi_function(function: &ItemFn) -> bool {
  matches!(function.vis, Visibility::Public(_))
    && function
      .sig
      .abi
      .as_ref()
      .is_some_and(is_c_abi)
    && has_no_mangle(function)
}

fn is_c_abi(abi: &Abi) -> bool {
  abi
    .name
    .as_ref()
    .is_some_and(|name| name.value() == "C")
}

fn has_no_mangle(function: &ItemFn) -> bool {
  function
    .attrs
    .iter()
    .filter(|attr| attr.path().is_ident("unsafe"))
    .any(|attr| {
      let mut found = false;
      let _ = attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("no_mangle") {
          found = true;
        }

        Ok(())
      });

      found
    })
}

fn node_type(ty: &Type) -> Result<&'static str> {
  const ERR: &str = "unsupported FFI type";

  let value = match ty {
    Type::Ptr(_) => "ffi.types.POINTER",
    Type::FnPtr(_) => "ffi.types.FUNCTION",
    Type::Path(path) => {
      let Some(segment) = path.path.segments.last() else { bail!(ERR) };

      #[allow(clippy::match_same_arms)]
      match segment.ident.to_string().as_str() {
        "i8" => "ffi.types.INT_8",
        "u8" => "ffi.types.UINT_8",
        "i16" => "ffi.types.INT_16",
        "u16" => "ffi.types.UINT_16",
        "i32" => "ffi.types.INT_32",
        "u32" => "ffi.types.UINT_32",
        "i64" => "ffi.types.INT_64",
        "u64" => "ffi.types.UINT_64",
        "f32" => "ffi.types.FLOAT_32",
        "f64" => "ffi.types.FLOAT_64",
        "bool" => "ffi.types.BOOL",
        "Status" => "ffi.types.INT_32",
        _ => bail!(ERR),
      }
    }
    _ => bail!(ERR),
  };

  Ok(value)
}
