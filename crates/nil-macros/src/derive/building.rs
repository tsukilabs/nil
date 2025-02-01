use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_building(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    mod __impl_building {
      use super::#name;
      use crate::building::{Building, BuildingLevel};

      impl Building for #name {
        fn level(&self) -> BuildingLevel {
          self.level
        }

        fn max_level(&self) -> BuildingLevel {
          Self::MAX_LEVEL
        }
      }
    }
  };

  stream.into()
}
