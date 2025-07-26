// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_owner(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    mod __impl_owner {
      use super::#name;
      use crate::npc::bot::BotId;
      use crate::npc::precursor::PrecursorId;
      use crate::player::PlayerId;

      impl #name {
        /// Returns the id of the bot to whom it belongs, if any.
        #[inline]
        pub fn bot(&self) -> Option<BotId> {
          if let Self::Bot { id } = self { Some(*id) } else { None }
        }

        /// Returns the id of the player to whom it belongs, if any.
        #[inline]
        pub fn player(&self) -> Option<&PlayerId> {
          if let Self::Player { id } = self { Some(id) } else { None }
        }

        /// Returns the id of the precursor to whom it belongs, if any.
        #[inline]
        pub fn precursor(&self) -> Option<PrecursorId> {
          if let Self::Precursor { id } = self { Some(*id) } else { None }
        }
      }

      impl From<BotId> for #name {
        fn from(id: BotId) -> Self {
          Self::Bot { id }
        }
      }

      impl From<PlayerId> for #name {
        fn from(id: PlayerId) -> Self {
          Self::Player { id }
        }
      }

      impl From<&PlayerId> for #name {
        fn from(id: &PlayerId) -> Self {
          Self::Player { id: id.clone() }
        }
      }

      impl From<PrecursorId> for #name {
        fn from(id: PrecursorId) -> Self {
          Self::Precursor { id }
        }
      }
    }
  };

  stream.into()
}
