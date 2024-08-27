#![feature(auto_traits)]

auto trait Trait1<'a> {}
// { dg-error ".E0567." "" { target *-*-* } .-1 }

fn f<'a>(x: &dyn Trait1<'a>)
{}

fn main() {
    f(&1);
}

