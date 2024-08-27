struct Foo<T, 'a>(&'a ());
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

