//@ run-rustfix

#[allow(unused)]
struct Struct<T>(T);

impl<T> Struct<T> where T:: std::fmt::Display {
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

fn main() {}

