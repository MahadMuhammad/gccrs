#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

trait Trait<T> {
    fn fnc<const N: usize = "">(&self) {} // { dg-error ".E0308." "" { target *-*-* } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    fn foo<const N: usize = { std::mem::size_of::<T>() }>(&self) {} // { dg-error "" "" { target *-*-* } }
}

fn main() {}

