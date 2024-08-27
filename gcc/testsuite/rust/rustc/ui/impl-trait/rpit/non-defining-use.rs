// Regression test for #111935 that non-defining uses of RPIT result in errors
#![allow(unconditional_recursion)]
fn foo<T>() -> impl Sized {
    let _: () = foo::<u8>(); // { dg-error ".E0792." "" { target *-*-* } }
}

fn bar<T>(val: T) -> impl Sized {
    let _: u8 = bar(0u8);
// { dg-error ".E0792." "" { target *-*-* } .-1 }
// { dg-error ".E0792." "" { target *-*-* } .-2 }
    val
}

fn main() {}

