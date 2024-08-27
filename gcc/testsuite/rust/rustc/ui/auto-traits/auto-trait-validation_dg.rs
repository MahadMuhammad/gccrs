#![feature(auto_traits)]
#![allow(dead_code)]

//@ run-rustfix

auto trait Generic<T> {}
// { dg-error ".E0567." "" { target *-*-* } .-1 }
auto trait Bound : Copy {}
// { dg-error ".E0568." "" { target *-*-* } .-1 }
auto trait LifetimeBound : 'static {}
// { dg-error ".E0568." "" { target *-*-* } .-1 }
auto trait MyTrait { fn foo() {} }
// { dg-error ".E0380." "" { target *-*-* } .-1 }
fn main() {}

