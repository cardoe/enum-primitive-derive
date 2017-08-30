// Copyright (c) 2017 Doug Goldstein <cardoe@cardoe.com>

// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// “Software”), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:

// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
// TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
// SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! This crate provides a custom derive `Primitive` that helps people
//! providing native Rust bindings to C code by allowing a C-like `enum`
//! declaration to convert to its primitve values and back from them. You
//! can selectively include `num_traits::ToPrimitive` and
//! `num_traits::FromPrimitive` to get these features.
//!
//! # Example
//!
//! ```rust
//! #[macro_use]
//! extern crate enum_primitive_derive;
//! extern crate num_traits;
//!
//! use num_traits::{FromPrimitive, ToPrimitive};
//!
//! #[derive(Primitive)]
//! enum Foo {
//!     Bar = 32,
//!     Dead = 42,
//!     Beef = 50,
//! }
//!
//! fn main() {
//!     assert_eq!(Foo::from_i32(32), Some(Foo::Bar));
//!     assert_eq!(Foo::from_i32(42), Some(Foo::Dead));
//!     assert_eq!(Foo::from_i64(50), Some(Foo::Beef));
//!     assert_eq!(Foo::from_isize(17), None);
//!     assert_eq!(Foo::Bar::to_i32(), Some(32));
//!     assert_eq!(Foo::Dead::to_isize(), Some(42));
//! }
//! ```

extern crate proc_macro;
extern crate num_traits;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

/// Provides implementation of `num_traits::ToPrimitive` and
/// `num_traits::FromPrimitive`
#[proc_macro_derive(Primitive)]
pub fn primitive(input: TokenStream) -> TokenStream {
	// Construct a string representation of the type definition
	let s = input.to_string();

	// Parse the string representation
	let ast = syn::parse_derive_input(&s).unwrap();

	// Build the impl
	let gen = impl_primitive(&ast);

	// Return the generated impl
	gen.parse().unwrap()
}

fn impl_primitive(ast: &syn::DeriveInput) -> quote::Tokens {
	let name = &ast.ident;

    // Check if derive(Primitive) was specified for a struct
    if let syn::Body::Enum(ref variant) = ast.body {

        let (var_u64, dis_u64): (Vec<_>, Vec<_>) = variant.iter().map(|v| {
            if v.discriminant.is_none() {
                panic!("#[derive(Primitive) requires C-like enums with \
                       discriminants for all enum variants");
            }
            (v.ident.clone(), v.discriminant.clone().unwrap())
        }).unzip();

        // quote!{} needs this to be a vec since its in #( )*
        let enum_u64 = vec![name.clone(); variant.len()];

        // can't reuse variables in quote!{} body
        let var_i64 = var_u64.clone();
        let dis_i64 = dis_u64.clone();
        let enum_i64 = enum_u64.clone();

        let to_name = name.clone();

        quote! {
            extern crate core;

            impl ::num_traits::FromPrimitive for #name {
                fn from_u64(val: u64) -> Option<Self> {
                    match val {
                        #( #dis_u64 => Some(#enum_u64::#var_u64), )*
                        _ => None,
                    }
                }

                fn from_i64(val: i64) -> Option<Self> {
                    match val {
                        #( #dis_i64 => Some(#enum_i64::#var_i64), )*
                        _ => None,
                    }
                }
            }

            impl ::num_traits::ToPrimitive for #to_name {
                fn to_u64(&self) -> Option<u64> {
                    Some(unsafe {
                        ::core::mem::transmute_copy::<Self, u64>(self)
                    })
                }

                fn to_i64(&self) -> Option<i64> {
                    Some(unsafe {
                        ::core::mem::transmute_copy::<Self, i64>(self)
                    })
                }
            }
		}
	} else {
        panic!("#[derive(Primitive)] is only valid for C-like enums");
    }
}
