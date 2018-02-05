#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{Fields, Item};
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
    match struct_.fields {
        // A field can only be named "bees" if it has a name, so we'll match
        // those fields and ignore the rest.
        Fields::Named(ref fields) => {
            fields.named.iter()
                .map(|field| {
                    // Check that the field has a name. I'm not sure how it could
                    // end up in `Fields::Named` if it didn't have a name, but what
                    // do I know?
                    if let Some(ident) = field.ident {
                        // You can get the string representation of a `syn::Ident` by
                        // using its `as_ref` or `to_string` methods.
                        ident.as_ref() == "bees"
                    } else {
                        false
                    }
                }).any(|x| x)
        }
        // Ignore unit structs or anonymous fields.
        _ => {
            false
        },
    }
}

/// Generate fun compiler errors.
fn light_it_up(struct_: &syn::ItemStruct) {
    if let Fields::Named(ref fields) = struct_.fields {
        // Piece together our exquisite error message.
        let bees = "🐝".repeat(17);
        let msg = "🐝   not the bees!!! NOT THE BEEEEEES!!! 🐝";
        // The `join` method places the provided string between the joined items,
        // so putting empty strings at the beginning and end will put extra
        // newline characters at the beginning and end of the error message.
        let bees_msg = ["", bees.as_str(), msg, bees.as_str(), ""].join("\n");
        // Find the field named "bees".
        fields.named.iter()
            .for_each(|field| {
                if let Some(ident) = field.ident {
                    if ident.as_ref() == "bees" {
                        // Deliver the error message.
                        ident.span().unstable()
                            .error(bees_msg.clone())
                            .emit();
                    }
                }
            });
    }
}
