#![feature(auto_traits)]

auto trait Trait1<'outer> {}
// { dg-error ".E0567." "" { target *-*-* } .-1 }

fn f<'a>(x: impl Trait1<'a>) {}

fn main() {
    f("");
}

