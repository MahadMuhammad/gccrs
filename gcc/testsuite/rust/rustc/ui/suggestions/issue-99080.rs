//@ aux-build:meow.rs

extern crate meow;

use meow::Meow;

fn needs_meow<T: Meow>(t: T) {}

fn main() {
    needs_meow(1usize);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

struct LocalMeow;

impl Meow for LocalMeow {}

