// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]

mod r#impl;

use proc_macro::TokenStream;
use syn::DeriveInput;

macro_rules! create_big_int_derive {
  ($target:ident) => {
    paste::paste! {
      #[proc_macro_derive([<BigInt $target:camel>])]
      pub fn [<derive_big_int_ $target:snake>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        r#impl::big_int::[<impl_big_int_ $target:snake>](&ast)
      }
    }
  };
}

create_big_int_derive!(u64);
create_big_int_derive!(usize);

#[proc_macro_derive(ConstDeref)]
pub fn derive_const_deref(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  r#impl::deref::impl_const_deref(&ast)
}

#[proc_macro_derive(ConstDerefMut)]
pub fn derive_const_deref_mut(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  r#impl::deref::impl_const_deref_mut(&ast)
}

macro_rules! create_math_derive {
  ($target:ident) => {
    paste::paste! {
      #[proc_macro_derive([<$target:camel Math>])]
      pub fn [<derive_ $target:snake _math>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        r#impl::math::[<impl_ $target:snake _math>](&ast)
      }

      #[proc_macro_derive([<$target:camel Add>])]
      pub fn [<derive_ $target:snake _add>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        r#impl::math::[<impl_ $target:snake _add>](&ast)
      }

      #[proc_macro_derive([<$target:camel AddAssign>])]
      pub fn [<derive_ $target:snake _add_assign>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        r#impl::math::[<impl_ $target:snake _add_assign>](&ast)
      }

      #[proc_macro_derive([<$target:camel Sub>])]
      pub fn [<derive_ $target:snake _sub>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        r#impl::math::[<impl_ $target:snake _sub>](&ast)
      }

      #[proc_macro_derive([<$target:camel SubAssign>])]
      pub fn [<derive_ $target:snake _sub_assign>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        r#impl::math::[<impl_ $target:snake _sub_assign>](&ast)
      }

      #[proc_macro_derive([<$target:camel Mul>])]
      pub fn [<derive_ $target:snake _mul>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        r#impl::math::[<impl_ $target:snake _mul>](&ast)
      }

      #[proc_macro_derive([<$target:camel MulAssign>])]
      pub fn [<derive_ $target:snake _mul_assign>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        r#impl::math::[<impl_ $target:snake _mul_assign>](&ast)
      }

      #[proc_macro_derive([<$target:camel Div>])]
      pub fn [<derive_ $target:snake _div>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        r#impl::math::[<impl_ $target:snake _div>](&ast)
      }

      #[proc_macro_derive([<$target:camel DivAssign>])]
      pub fn [<derive_ $target:snake _div_assign>](input: TokenStream) -> TokenStream {
        let ast = syn::parse::<DeriveInput>(input).unwrap();
        r#impl::math::[<impl_ $target:snake _div_assign>](&ast)
      }
    }
  };
}

create_math_derive!(f64);
