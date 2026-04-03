// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::DeriveInput;

macro_rules! decl {
  ($num:ident, $name:expr) => {
    paste::paste! {
      pub fn [<impl_ $num:snake _ops>](ast: &DeriveInput) -> TokenStream {
        let add = [<impl_ $num:snake _add>](ast);
        let sub = [<impl_ $num:snake _sub>](ast);
        let mul = [<impl_ $num:snake _mul>](ast);
        let div = [<impl_ $num:snake _div>](ast);

        add
          .into_iter()
          .chain(sub)
          .chain(mul)
          .chain(div)
          .collect()
      }

      pub fn [<impl_ $num:snake _add>](ast: &DeriveInput) -> TokenStream {
        impl_add(ast, &Ident::new($name, Span::call_site()))
      }

      pub fn [<impl_ $num:snake _sub>](ast: &DeriveInput) -> TokenStream {
        impl_sub(ast, &Ident::new($name, Span::call_site()))
      }

      pub fn [<impl_ $num:snake _mul>](ast: &DeriveInput) -> TokenStream {
        impl_mul(ast, &Ident::new($name, Span::call_site()))
      }

      pub fn [<impl_ $num:snake _div>](ast: &DeriveInput) -> TokenStream {
        impl_div(ast, &Ident::new($name, Span::call_site()))
      }
    }
  };
}

decl!(f64, "f64");

fn impl_add(ast: &DeriveInput, num: &Ident) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    impl std::ops::Add<#num> for #name {
      type Output = #num;

      fn add(self, rhs: #num) -> Self::Output {
        #num::from(self) + rhs
      }
    }

    impl std::ops::Add<#name> for #num {
      type Output = #num;

      fn add(self, rhs: #name) -> Self::Output {
        self + #num::from(rhs)
      }
    }
  };

  stream.into()
}

fn impl_sub(ast: &DeriveInput, num: &Ident) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    impl std::ops::Sub<#num> for #name {
      type Output = #num;

      fn sub(self, rhs: #num) -> Self::Output {
        #num::from(self) - rhs
      }
    }

    impl std::ops::Sub<#name> for #num {
      type Output = #num;

      fn sub(self, rhs: #name) -> Self::Output {
        self - #num::from(rhs)
      }
    }
  };

  stream.into()
}

fn impl_mul(ast: &DeriveInput, num: &Ident) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    impl std::ops::Mul<#num> for #name {
      type Output = #num;

      fn mul(self, rhs: #num) -> Self::Output {
        #num::from(self) * rhs
      }
    }

    impl std::ops::Mul<#name> for #num {
      type Output = #num;

      fn mul(self, rhs: #name) -> Self::Output {
        self * #num::from(rhs)
      }
    }
  };

  stream.into()
}

fn impl_div(ast: &DeriveInput, num: &Ident) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    impl std::ops::Div<#num> for #name {
      type Output = #num;

      fn div(self, rhs: #num) -> Self::Output {
        #num::from(self) / rhs
      }
    }

    impl std::ops::Div<#name> for #num {
      type Output = #num;

      fn div(self, rhs: #name) -> Self::Output {
        self / #num::from(rhs)
      }
    }
  };

  stream.into()
}
