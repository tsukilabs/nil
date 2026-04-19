// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_precursor(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    mod __impl_precursor {
      use super::#name;
      use crate::continent::Coord;
      use crate::ethic::Ethics;
      use crate::npc::precursor::{Precursor, PrecursorBox, PrecursorId};
      use crate::resources::Resources;
      use crate::resources::influence::Influence;

      impl #name {
        pub fn boxed(&self) -> PrecursorBox {
          PrecursorBox::from(self.clone())
        }
      }

      impl Precursor for #name {
        fn id(&self) -> PrecursorId {
          Self::ID
        }

        fn ethics(&self) -> &Ethics {
          &Self::ETHICS
        }

        fn origin(&self) -> Coord {
          self.origin
        }

        fn resources(&self) -> &Resources {
          &self.resources
        }

        fn resources_mut(&mut self) -> &mut Resources {
          &mut self.resources
        }

        fn influence(&self) -> Influence {
          self.influence
        }
      }
    }
  };

  stream.into()
}
