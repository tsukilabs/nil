// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use proc_macro::TokenStream;
use quote::quote;
use std::iter;
use syn::DeriveInput;

pub fn impl_f64_math(ast: &DeriveInput) -> TokenStream {
  let add = impl_add(ast);
  let sub = impl_sub(ast);
  let mul = impl_mul(ast);
  let div = impl_div(ast);

  let add_assign = impl_add_assign(ast);
  let sub_assign = impl_sub_assign(ast);
  let mul_assign = impl_mul_assign(ast);
  let div_assign = impl_div_assign(ast);

  iter::empty()
    .chain(add)
    .chain(sub)
    .chain(mul)
    .chain(div)
    .chain(add_assign)
    .chain(sub_assign)
    .chain(mul_assign)
    .chain(div_assign)
    .collect()
}

fn impl_add(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    const impl ::core::ops::Add<f64> for #name {
      type Output = f64;

      fn add(self, rhs: f64) -> Self::Output {
        f64::from(self) + rhs
      }
    }

    #[automatically_derived]
    const impl ::core::ops::Add<#name> for f64 {
      type Output = f64;

      fn add(self, rhs: #name) -> Self::Output {
        self + f64::from(rhs)
      }
    }
  };

  stream.into()
}

fn impl_add_assign(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    const impl ::core::ops::AddAssign<#name> for f64 {
      fn add_assign(&mut self, rhs: #name) {
        *self = *self + f64::from(rhs);
      }
    }
  };

  stream.into()
}

fn impl_sub(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    const impl ::core::ops::Sub<f64> for #name {
      type Output = f64;

      fn sub(self, rhs: f64) -> Self::Output {
        f64::from(self) - rhs
      }
    }

    #[automatically_derived]
    const impl ::core::ops::Sub<#name> for f64 {
      type Output = f64;

      fn sub(self, rhs: #name) -> Self::Output {
        self - f64::from(rhs)
      }
    }
  };

  stream.into()
}

fn impl_sub_assign(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    const impl ::core::ops::SubAssign<#name> for f64 {
      fn sub_assign(&mut self, rhs: #name) {
        *self = *self - f64::from(rhs);
      }
    }
  };

  stream.into()
}

fn impl_mul(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    const impl ::core::ops::Mul<f64> for #name {
      type Output = f64;

      fn mul(self, rhs: f64) -> Self::Output {
        f64::from(self) * rhs
      }
    }

    #[automatically_derived]
    const impl ::core::ops::Mul<#name> for f64 {
      type Output = f64;

      fn mul(self, rhs: #name) -> Self::Output {
        self * f64::from(rhs)
      }
    }
  };

  stream.into()
}

fn impl_mul_assign(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    const impl ::core::ops::MulAssign<#name> for f64 {
      fn mul_assign(&mut self, rhs: #name) {
        *self = *self * f64::from(rhs);
      }
    }
  };

  stream.into()
}

fn impl_div(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    const impl ::core::ops::Div<f64> for #name {
      type Output = f64;

      fn div(self, rhs: f64) -> Self::Output {
        f64::from(self) / rhs
      }
    }

    #[automatically_derived]
    const impl ::core::ops::Div<#name> for f64 {
      type Output = f64;

      fn div(self, rhs: #name) -> Self::Output {
        self / f64::from(rhs)
      }
    }
  };

  stream.into()
}

fn impl_div_assign(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    const impl ::core::ops::DivAssign<#name> for f64 {
      fn div_assign(&mut self, rhs: #name) {
        *self = *self / f64::from(rhs);
      }
    }
  };

  stream.into()
}
