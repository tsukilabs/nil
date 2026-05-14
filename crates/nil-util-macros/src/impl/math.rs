// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::DeriveInput;

macro_rules! decl_math {
  ($num:ident, $num_name:literal) => {
    paste::paste! {
      pub fn [<impl_ $num:snake _math>](ast: &DeriveInput) -> TokenStream {
        let add = [<impl_ $num:snake _add>](ast);
        let sub = [<impl_ $num:snake _sub>](ast);
        let mul = [<impl_ $num:snake _mul>](ast);
        let div = [<impl_ $num:snake _div>](ast);

        let add_assign = [<impl_ $num:snake _add_assign>](ast);
        let sub_assign = [<impl_ $num:snake _sub_assign>](ast);
        let mul_assign = [<impl_ $num:snake _mul_assign>](ast);
        let div_assign = [<impl_ $num:snake _div_assign>](ast);

        add
          .into_iter()
          .chain(sub)
          .chain(mul)
          .chain(div)
          .chain(add_assign)
          .chain(sub_assign)
          .chain(mul_assign)
          .chain(div_assign)
          .collect()
      }

      pub fn [<impl_ $num:snake _add>](ast: &DeriveInput) -> TokenStream {
        impl_add(ast, &Ident::new($num_name, Span::call_site()))
      }

      pub fn [<impl_ $num:snake _add_assign>](ast: &DeriveInput) -> TokenStream {
        impl_add_assign(ast, &Ident::new($num_name, Span::call_site()))
      }

      pub fn [<impl_ $num:snake _sub>](ast: &DeriveInput) -> TokenStream {
        impl_sub(ast, &Ident::new($num_name, Span::call_site()))
      }

      pub fn [<impl_ $num:snake _sub_assign>](ast: &DeriveInput) -> TokenStream {
        impl_sub_assign(ast, &Ident::new($num_name, Span::call_site()))
      }

      pub fn [<impl_ $num:snake _mul>](ast: &DeriveInput) -> TokenStream {
        impl_mul(ast, &Ident::new($num_name, Span::call_site()))
      }

      pub fn [<impl_ $num:snake _mul_assign>](ast: &DeriveInput) -> TokenStream {
        impl_mul_assign(ast, &Ident::new($num_name, Span::call_site()))
      }

      pub fn [<impl_ $num:snake _div>](ast: &DeriveInput) -> TokenStream {
        impl_div(ast, &Ident::new($num_name, Span::call_site()))
      }

      pub fn [<impl_ $num:snake _div_assign>](ast: &DeriveInput) -> TokenStream {
        impl_div_assign(ast, &Ident::new($num_name, Span::call_site()))
      }
    }
  };
}

decl_math!(f64, "f64");

fn impl_add(ast: &DeriveInput, num: &Ident) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    impl const ::core::ops::Add<#num> for #name {
      type Output = #num;

      fn add(self, rhs: #num) -> Self::Output {
        #num::from(self) + rhs
      }
    }

    #[automatically_derived]
    impl const ::core::ops::Add<#name> for #num {
      type Output = #num;

      fn add(self, rhs: #name) -> Self::Output {
        self + #num::from(rhs)
      }
    }
  };

  stream.into()
}

fn impl_add_assign(ast: &DeriveInput, num: &Ident) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    impl const ::core::ops::AddAssign<#name> for #num {
      fn add_assign(&mut self, rhs: #name) {
        *self = *self + #num::from(rhs);
      }
    }
  };

  stream.into()
}

fn impl_sub(ast: &DeriveInput, num: &Ident) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    impl const ::core::ops::Sub<#num> for #name {
      type Output = #num;

      fn sub(self, rhs: #num) -> Self::Output {
        #num::from(self) - rhs
      }
    }

    #[automatically_derived]
    impl const ::core::ops::Sub<#name> for #num {
      type Output = #num;

      fn sub(self, rhs: #name) -> Self::Output {
        self - #num::from(rhs)
      }
    }
  };

  stream.into()
}

fn impl_sub_assign(ast: &DeriveInput, num: &Ident) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    impl const ::core::ops::SubAssign<#name> for #num {
      fn sub_assign(&mut self, rhs: #name) {
        *self = *self - #num::from(rhs);
      }
    }
  };

  stream.into()
}

fn impl_mul(ast: &DeriveInput, num: &Ident) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    impl const ::core::ops::Mul<#num> for #name {
      type Output = #num;

      fn mul(self, rhs: #num) -> Self::Output {
        #num::from(self) * rhs
      }
    }

    #[automatically_derived]
    impl const ::core::ops::Mul<#name> for #num {
      type Output = #num;

      fn mul(self, rhs: #name) -> Self::Output {
        self * #num::from(rhs)
      }
    }
  };

  stream.into()
}

fn impl_mul_assign(ast: &DeriveInput, num: &Ident) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    impl const ::core::ops::MulAssign<#name> for #num {
      fn mul_assign(&mut self, rhs: #name) {
        *self = *self * #num::from(rhs);
      }
    }
  };

  stream.into()
}

fn impl_div(ast: &DeriveInput, num: &Ident) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    impl const ::core::ops::Div<#num> for #name {
      type Output = #num;

      fn div(self, rhs: #num) -> Self::Output {
        #num::from(self) / rhs
      }
    }

    #[automatically_derived]
    impl const ::core::ops::Div<#name> for #num {
      type Output = #num;

      fn div(self, rhs: #name) -> Self::Output {
        self / #num::from(rhs)
      }
    }
  };

  stream.into()
}

fn impl_div_assign(ast: &DeriveInput, num: &Ident) -> TokenStream {
  let name = &ast.ident;
  let stream = quote! {
    #[automatically_derived]
    impl const ::core::ops::DivAssign<#name> for #num {
      fn div_assign(&mut self, rhs: #name) {
        *self = *self / #num::from(rhs);
      }
    }
  };

  stream.into()
}
