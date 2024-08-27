#![feature(fn_traits)]
#![feature(unboxed_closures)]

struct S;

impl Fn(u32) -> u32 for S {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
    fn call(&self) -> u32 {
// { dg-error ".E0050." "" { target *-*-* } .-1 }
        5
    }
}

fn main() {}

