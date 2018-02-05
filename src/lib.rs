#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn wickerman(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    // just return the input
    input
}
