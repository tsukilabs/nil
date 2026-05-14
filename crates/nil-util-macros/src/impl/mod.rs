// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod big_int;
pub mod deref;
pub mod math;

use syn::{Data, DeriveInput, Fields, Ident, Type};

fn get_tuple_struct_inner_ident(input: &DeriveInput) -> &Ident {
  if let Data::Struct(r#struct) = &input.data
    && let Fields::Unnamed(fields) = &r#struct.fields
    && let Some(field) = fields.unnamed.first()
    && let Type::Path(ty_path) = &field.ty
    && let Some(ident) = ty_path.path.get_ident()
  {
    ident
  } else {
    panic!("");
  }
}
