#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn wickerman(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    // Convert the `proc_macro::TokenStream` into a `proc_macro2::TokenStream` to get the
    // span information from the compiler.
    let input: proc_macro2::TokenStream = input.into();

    // Convert the `proc_macro2::TokenStream` back into a `proc_macro::TokenStream`.
    let output: TokenStream = input.into();

    // Return the `TokenStream`.
    output
}
