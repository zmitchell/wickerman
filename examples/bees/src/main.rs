#![feature(proc_macro)]

extern crate wickerman;
use wickerman::wickerman;

/// This struct shouldn't raise any errors because it doesn't have a field
/// named "bees", or any named fields at all for that matter.
#[wickerman]
struct Foo(i32);

/// This is where the action will happen.
#[wickerman]
struct Bar<'a> {
    baz: i32,
    bees: String,
    qux: &'a str,
    spam: &'a Foo,
    eggs: &'a [u8],
}

/// This is only here so that the crate will run as a binary crate
fn main() {
    println!("Hello, world!");
}
