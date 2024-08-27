// Regression test for #80779.

pub struct T<'a>(&'a str);

pub fn f<'a>(val: T<'a>) -> _ {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
    g(val)
}

pub fn g(_: T<'static>) -> _ {}
// { dg-error ".E0121." "" { target *-*-* } .-1 }

fn main() {}

