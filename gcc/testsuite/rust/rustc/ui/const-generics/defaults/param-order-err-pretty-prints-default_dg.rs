struct Foo<const M: usize = 10, 'a>(&'a u32);
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

