use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_unit(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    mod __impl_unit {
      use super::#name;
      use nil_core::{Unit, UnitBox, UnitId, UnitKind, UnitStats};

      impl #name {
        pub fn new_boxed(amount: u32) -> UnitBox {
          UnitBox::new(Box::new(Self { amount }))
        }
      }

      impl Unit for #name {
        fn id(&self) -> UnitId {
          Self::ID
        }

        fn kind(&self) -> UnitKind {
          Self::KIND
        }

        fn amount(&self) -> u32 {
          self.amount
        }

        fn stats(&self) -> UnitStats {
          Self::STATS
        }
      }
    }
  };

  stream.into()
}
