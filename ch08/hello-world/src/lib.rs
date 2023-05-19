#![crate_type = "proc-macro"]
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn say_hello_world(_item: TokenStream) -> TokenStream {
    "println!(\"hello world\")".parse().unwrap()
}
