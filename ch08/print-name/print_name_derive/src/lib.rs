// This is based on the heapsize example from the syn crate, at
// https://github.com/dtolnay/syn/tree/master/examples/heapsize.
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, DeriveInput, GenericParam, Generics,
};

#[proc_macro_derive(PrintName)]
pub fn print_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, type_generics, where_clause) =
        generics.split_for_impl();

    let name = input.ident;

    let expanded = quote! {
        impl #impl_generics print_name::PrintName for #name #type_generics #where_clause {
            fn name() -> &'static str {
                stringify!(#name)
            }
        }
    };

    TokenStream::from(expanded)
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(print_name::PrintName));
        }
    }
    generics
}
