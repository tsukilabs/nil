// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::DeriveInput;

pub fn impl_report(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let kind = Ident::new(name.to_string().trim_suffix("Report"), Span::call_site());

  let stream = quote! {
    mod __impl_report {
      use super::#name;
      use crate::report::{Report, ReportId, ReportKind};
      use jiff::Zoned;
      use std::sync::Arc;

      impl Report for #name {
        fn id(&self) -> ReportId {
          self.id
        }

        fn timestamp(&self) -> &Zoned {
          &self.timestamp
        }
      }

      impl From<#name> for ReportKind {
        fn from(report: #name) -> Self {
          ReportKind::#kind { report: Arc::new(report) }
        }
      }
    }
  };

  stream.into()
}
