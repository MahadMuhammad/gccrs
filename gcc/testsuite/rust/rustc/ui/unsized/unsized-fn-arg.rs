//@ run-rustfix
#![crate_type="lib"]
#![allow(unused)]

fn f<T: ?Sized>(t: T) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }

