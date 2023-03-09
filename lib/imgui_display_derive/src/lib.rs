#![recursion_limit = "128"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use syn::{DeriveInput, Type};

#[proc_macro_derive(ImGuiDisplay)]
pub fn imgui_display_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_macro(&ast)
}

fn impl_macro(ast: &DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let generics = &ast.generics;
    let where_clause = &ast.generics.where_clause;

    let display_implementations = match &ast.data {
        syn::Data::Enum(_) => panic!("Not applicable for enum type!"),
        syn::Data::Union(_) => panic!("Not applicable for union type!"),
        syn::Data::Struct(syn::DataStruct { fields, .. }) => generate_display_implementation(fields),
    };

    let gen = quote! {
        impl #ident #generics #where_clause {
            pub fn display() {
            }
        }
    };

    panic!("NO!");

    gen.into()
}

fn generate_display_implementation(fields: &syn::Fields) -> TokenStream {
    fields.iter()
        .map(generate_display_implementation_field)
        .collect()
}

fn generate_display_implementation_field(field: &syn::Field) -> TokenStream {
    match &field.ty {
        Type::Path(path) => panic!("{:#?}", path),
        _ => todo!("Not yet implemented!"),
    }

    (quote! { "Foobar" }).into()
}
