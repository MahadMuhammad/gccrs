//@ run-rustfix

#[allow(unused)]
struct Struct<T>(T);

impl<T: Iterator> Struct<T> where <T as std:: iter::Iterator>::Item:: std::fmt::Display {
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

fn main() {}

