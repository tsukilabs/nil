// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_ruler(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    mod __impl_ruler {
      use super::#name;
      use crate::npc::bot::BotId;
      use crate::npc::precursor::PrecursorId;
      use crate::player::PlayerId;
      use crate::ruler::Ruler;

      impl Ruler for #name {
        /// Returns the id of the bot to whom it belongs, if any.
        #[inline]
        fn bot(&self) -> Option<&BotId> {
          if let Self::Bot { id } = self { Some(id) } else { None }
        }

        /// Returns the id of the player to whom it belongs, if any.
        #[inline]
        fn player(&self) -> Option<&PlayerId> {
          if let Self::Player { id } = self { Some(id) } else { None }
        }

        /// Returns the id of the precursor to whom it belongs, if any.
        #[inline]
        fn precursor(&self) -> Option<PrecursorId> {
          if let Self::Precursor { id } = self { Some(*id) } else { None }
        }
      }

      impl<T: Ruler> PartialEq<T> for #name {
        fn eq(&self, other: &T) -> bool {
          match self {
            Self::Bot { id } => other.is_bot_and(|it| it == id),
            Self::Player { id } => other.is_player_and(|it| it == id),
            Self::Precursor { id } => other.is_precursor_and(|it| it == *id),
          }
        }
      }

      impl From<BotId> for #name {
        fn from(id: BotId) -> Self {
          Self::Bot { id }
        }
      }

      impl From<&BotId> for #name {
        fn from(id: &BotId) -> Self {
          Self::Bot { id: id.clone() }
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
