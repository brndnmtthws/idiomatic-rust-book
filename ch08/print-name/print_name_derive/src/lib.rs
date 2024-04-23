// This is based on the heapsize example from the syn crate, at
// https://github.com/dtolnay/syn/tree/master/examples/heapsize.
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PrintName)]
pub fn print_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let (impl_generics, type_generics, where_clause) =
        input.generics.split_for_impl();

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
