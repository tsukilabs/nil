// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]

mod big_int;
mod ops;

use proc_macro::TokenStream;
use syn::DeriveInput;

macro_rules! create_big_int_derive {
  ($target:ident) => {
    paste::paste! {
      #[proc_macro_derive([<BigInt $target:camel>])]
      pub fn [<derive_big_int_ $target:snake>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        big_int::[<impl_big_int_ $target:snake>](&ast)
      }
    }
  };
}

create_big_int_derive!(u64);
create_big_int_derive!(usize);

macro_rules! create_ops_derive {
  ($target:ident) => {
    paste::paste! {
      #[proc_macro_derive([<$target:camel Math>])]
      pub fn [<derive_ $target:snake _math>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        ops::[<impl_ $target:snake _math>](&ast)
      }

      #[proc_macro_derive([<$target:camel Add>])]
      pub fn [<derive_ $target:snake _add>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        ops::[<impl_ $target:snake _add>](&ast)
      }

      #[proc_macro_derive([<$target:camel AddAssign>])]
      pub fn [<derive_ $target:snake _add_assign>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        ops::[<impl_ $target:snake _add_assign>](&ast)
      }

      #[proc_macro_derive([<$target:camel Sub>])]
      pub fn [<derive_ $target:snake _sub>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        ops::[<impl_ $target:snake _sub>](&ast)
      }

      #[proc_macro_derive([<$target:camel SubAssign>])]
      pub fn [<derive_ $target:snake _sub_assign>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        ops::[<impl_ $target:snake _sub_assign>](&ast)
      }

      #[proc_macro_derive([<$target:camel Mul>])]
      pub fn [<derive_ $target:snake _mul>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        ops::[<impl_ $target:snake _mul>](&ast)
      }

      #[proc_macro_derive([<$target:camel MulAssign>])]
      pub fn [<derive_ $target:snake _mul_assign>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        ops::[<impl_ $target:snake _mul_assign>](&ast)
      }

      #[proc_macro_derive([<$target:camel Div>])]
      pub fn [<derive_ $target:snake _div>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        ops::[<impl_ $target:snake _div>](&ast)
      }

      #[proc_macro_derive([<$target:camel DivAssign>])]
      pub fn [<derive_ $target:snake _div_assign>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        ops::[<impl_ $target:snake _div_assign>](&ast)
      }
    }
  };
}

create_ops_derive!(f64);
