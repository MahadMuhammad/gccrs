// Regression test for the ICE caused by the example in
// https://github.com/rust-lang/rust/issues/86053#issuecomment-855672258

#![feature(c_variadic)]

trait H<T> {}

unsafe extern "C" fn ordering4<'a, F: H<&'static &'a ()>>(_: (), ...) {}
// { dg-error ".E0491." "" { target *-*-* } .-1 }

fn main() {}

