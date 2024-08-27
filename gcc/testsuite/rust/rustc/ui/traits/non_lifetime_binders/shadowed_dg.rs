#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

fn function<T>() where for<T> (): Sized {}
// { dg-error ".E0403." "" { target *-*-* } .-1 }

struct Struct<T>(T) where for<T> (): Sized;
// { dg-error ".E0403." "" { target *-*-* } .-1 }

impl<T> Struct<T> {
    fn method() where for<T> (): Sized {}
// { dg-error ".E0403." "" { target *-*-* } .-1 }
}

fn repeated() where for<T, T> (): Sized {}
// { dg-error ".E0403." "" { target *-*-* } .-1 }

fn main() {}

