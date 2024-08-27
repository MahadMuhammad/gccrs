#![crate_type = "lib"]

extern fn none_fn(x: bool) -> i32;
// { dg-error "" "" { target *-*-* } .-1 }
extern "C" fn c_fn(x: bool) -> i32;
// { dg-error "" "" { target *-*-* } .-1 }

