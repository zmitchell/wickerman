#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::Item;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn wickerman(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    // Convert the `proc_macro::TokenStream` into a `proc_macro2::TokenStream` to get the
    // span information from the compiler.
    let input: proc_macro2::TokenStream = input.into();

    // Parse the `TokenStream` into a syntax tree, specifically an `Item`. An `Item` is a
    // syntax item that can appear at the module level i.e. a function definition, a struct
    // or enum definition, etc.
    let item: syn::Item = syn::parse2(input).expect("failed to parse input into `syn::Item`");

    // Match on the parsed item and respond accordingly.
    match item {
        // If the attribute was applied to a struct, we're going to do some more work
        // to figure out if there's a field named "bees". It's important to take a reference
        // to `struct_item`, otherwise you partially move `item`.
        Item::Struct(ref struct_item) => {
            if has_bees(struct_item) {
                light_it_up(struct_item);
            }
        },

        // If the attribute was applied to any other kind of item, we want to generate a
        // compiler error.
        _ => {
            // This is how you generate a compiler error. You can also generate a "note",
            // or a "warning".
            item.span().unstable()
                .error("This is not a struct")
                .emit();
        },
    }

    // Use `quote` to convert the syntax tree back into tokens so we can return them. Note
    // that the tokens we're returning at this point are still just the input, we've simply
    // converted it between a few different forms.
    let output = quote!{ #item };
    output.into()
}

/// Determine if the struct has a field named "bees".
fn has_bees(struct_: &syn::ItemStruct) -> bool {
    unimplemented!()
}

/// Generate fun compiler errors.
fn light_it_up(struct_: &syn::ItemStruct) {
    unimplemented!()
}
