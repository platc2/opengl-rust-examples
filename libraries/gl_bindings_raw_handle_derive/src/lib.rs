extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(RawHandle)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(ref data) = input.data {
        if let syn::Fields::Unnamed(ref fields) = data.fields {
            if fields.unnamed.len() == 1 {
                let name = input.ident;
                let raw_type = &fields.unnamed.first()
                    .unwrap().ty;

                return TokenStream::from(quote!(
                    impl gl::RawHandle<#raw_type> for #name {
                        unsafe fn raw_handle(&self) -> #raw_type { self.0 }

                        unsafe fn from_raw(value: #raw_type) -> Self { Self(value) }
                    }
                ));
            }
        }
    }

    TokenStream::from(
        syn::Error::new(
            input.ident.span(),
            "Derivation of RawHandle only possible for single-element tuple structs!",
        ).to_compile_error()
    )
}
